use std::fs;
use std::sync::{Arc, Mutex};

use rusqlite::{params, Connection, OptionalExtension, Result as RusqliteResult};
use tauri::{AppHandle, Manager};

use super::types::IndexedFile;
use crate::error::AppError;

#[derive(Clone)]
pub struct FileSearchManager {
    db: Arc<Mutex<Connection>>,
}

impl FileSearchManager {
    pub fn new(app_handle: AppHandle) -> Result<Self, AppError> {
        let data_dir = app_handle
            .path()
            .app_local_data_dir()
            .map_err(|_| AppError::DirectoryNotFound)?;

        if !data_dir.exists() {
            fs::create_dir_all(&data_dir).map_err(|e| AppError::FileSearch(e.to_string()))?;
        }

        let db_path = data_dir.join("file_search.sqlite");
        let db = Connection::open(db_path)?;

        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }

    pub fn init_db(&self) -> RusqliteResult<()> {
        let db = self.db.lock().unwrap();

        db.execute(
            "CREATE TABLE IF NOT EXISTS file_index (
                path TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                parent_path TEXT NOT NULL,
                file_type TEXT NOT NULL,
                last_modified INTEGER NOT NULL
            )",
            [],
        )?;

        db.execute(
            "CREATE VIRTUAL TABLE IF NOT EXISTS file_index_fts
             USING fts5(name, content='file_index', content_rowid='rowid', tokenize = 'porter unicode61')",
            [],
        )?;

        db.execute(
            "CREATE TRIGGER IF NOT EXISTS file_index_after_insert
             AFTER INSERT ON file_index
             BEGIN
                INSERT INTO file_index_fts(rowid, name) VALUES (new.rowid, new.name);
             END;",
            [],
        )?;

        db.execute(
            "CREATE TRIGGER IF NOT EXISTS file_index_after_delete
             AFTER DELETE ON file_index
             BEGIN
                INSERT INTO file_index_fts(file_index_fts, rowid, name) VALUES ('delete', old.rowid, old.name);
             END;",
            [],
        )?;

        db.execute(
            "CREATE TRIGGER IF NOT EXISTS file_index_after_update
             AFTER UPDATE ON file_index
             BEGIN
                INSERT INTO file_index_fts(file_index_fts, rowid, name) VALUES ('delete', old.rowid, old.name);
                INSERT INTO file_index_fts(rowid, name) VALUES (new.rowid, new.name);
             END;",
            [],
        )?;

        Ok(())
    }

    pub fn add_file(&self, file: &IndexedFile) -> Result<(), AppError> {
        let db = self.db.lock().unwrap();
        db.execute(
            "INSERT OR REPLACE INTO file_index (path, name, parent_path, file_type, last_modified)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                file.path,
                file.name,
                file.parent_path,
                file.file_type,
                file.last_modified
            ],
        )?;
        Ok(())
    }

    pub fn remove_file(&self, path: &str) -> Result<(), AppError> {
        let db = self.db.lock().unwrap();
        db.execute("DELETE FROM file_index WHERE path = ?1", params![path])?;
        Ok(())
    }

    pub fn get_file_last_modified(&self, path: &str) -> Result<Option<i64>, AppError> {
        let db = self.db.lock().unwrap();
        let last_modified: Result<Option<i64>, rusqlite::Error> = db
            .query_row(
                "SELECT last_modified FROM file_index WHERE path = ?1",
                params![path],
                |row| row.get(0),
            )
            .optional();

        Ok(last_modified?)
    }

    pub fn search_files(&self, term: &str, limit: u32) -> Result<Vec<IndexedFile>, AppError> {
        let db = self.db.lock().unwrap();
        let mut stmt = db.prepare(
            "SELECT t1.path, t1.name, t1.parent_path, t1.file_type, t1.last_modified
             FROM file_index t1 JOIN file_index_fts t2 ON t1.rowid = t2.rowid
             WHERE t2.name MATCH ?1
             ORDER BY t1.last_modified DESC
             LIMIT ?2",
        )?;

        let search_term = format!("\"{}\"*", term);
        let files_iter = stmt.query_map(params![search_term, limit], |row| {
            Ok(IndexedFile {
                path: row.get(0)?,
                name: row.get(1)?,
                parent_path: row.get(2)?,
                file_type: row.get(3)?,
                last_modified: row.get(4)?,
            })
        })?;

        files_iter
            .collect::<RusqliteResult<Vec<_>>>()
            .map_err(|e| e.into())
    }
}
