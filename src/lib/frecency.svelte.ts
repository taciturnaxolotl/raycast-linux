import { invoke } from '@tauri-apps/api/core';

type FrecencyDataItem = {
	itemId: string;
	useCount: number;
	lastUsedAt: number;
};

class FrecencyStore {
	data = $state<FrecencyDataItem[]>([]);
	isLoading = $state(true);

	constructor() {
		this.fetchData();
	}

	async fetchData() {
		this.isLoading = true;
		try {
			this.data = await invoke<FrecencyDataItem[]>('get_frecency_data');
		} catch (e) {
			console.error('Failed to fetch frecency data:', e);
			this.data = [];
		} finally {
			this.isLoading = false;
		}
	}

	async recordUsage(itemId: string) {
		try {
			await invoke('record_usage', { itemId });
			this.fetchData();
		} catch (e) {
			console.error(`Failed to record usage for ${itemId}:`, e);
		}
	}
}

export const frecencyStore = new FrecencyStore();
