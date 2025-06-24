import { invoke } from '@tauri-apps/api/core';

type App = { name: string; comment?: string; exec: string; icon_path?: string };

class AppsStore {
	apps = $state<App[]>([]);
	isLoading = $state(true);

	constructor() {
		this.fetchApps();
	}

	async fetchApps() {
		this.isLoading = true;
		try {
			this.apps = await invoke<App[]>('get_installed_apps');
		} catch (e) {
			console.error('Failed to fetch installed apps:', e);
			this.apps = [];
		} finally {
			this.isLoading = false;
		}
	}
}

export const appsStore = new AppsStore();
