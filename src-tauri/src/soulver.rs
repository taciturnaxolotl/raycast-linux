use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn initialize(soulver_core_path: &str) {
    INIT.call_once(|| {
        let resources_path_str = format!("{}/SoulverCore_SoulverCore.resources", soulver_core_path);
        let resources_path_cstr = CString::new(resources_path_str).expect("CString::new failed");

        unsafe {
            initialize_soulver(resources_path_cstr.as_ptr());
        }
    });
}

#[link(name = "SoulverWrapper", kind = "dylib")]
extern "C" {
    fn initialize_soulver(resourcesPath: *const c_char);
    fn evaluate(expression: *const c_char) -> *mut c_char;
    fn free_string(ptr: *mut c_char);
}

#[tauri::command]
pub fn calculate_soulver(expression: String) -> Result<String, String> {
    let c_expression = CString::new(expression).map_err(|e| e.to_string())?;

    // we need to use unsafe because we are calling a foreign function
    let result_ptr = unsafe { evaluate(c_expression.as_ptr()) };

    if result_ptr.is_null() {
        return Err("Evaluation failed, received null pointer from Swift.".to_string());
    }

    let result_string = unsafe {
        // convert C string back to Rust String
        let c_result = CStr::from_ptr(result_ptr);
        let str_slice = c_result.to_str().map_err(|e| e.to_string())?;
        str_slice.to_owned()
    };

    // free memory that was allocated by Swift's strdup()
    unsafe { free_string(result_ptr) };

    Ok(result_string)
}
