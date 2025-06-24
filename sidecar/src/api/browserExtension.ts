import { sendRequest } from './rpc';

type Tab = {
	active: boolean;
	id: number;
	url: string;
	favicon?: string;
	title?: string;
};

type RawTab = {
	tabId: number;
	url: string;
	title?: string;
	favicon?: string;
	active: boolean;
};

const sendBrowserRequest = <T>(method: string, params: unknown) => {
	return sendRequest<T>('browser-extension-request', { method, params });
};

export const BrowserExtensionAPI = {
	async getTabs(): Promise<Tab[]> {
		const result = await sendBrowserRequest<{ value: RawTab[] }>('getTabs', {});
		return result.value.map((tab) => ({
			id: tab.tabId,
			url: tab.url,
			title: tab.title,
			favicon: tab.favicon,
			active: tab.active
		}));
	},
	async getContent(options?: {
		cssSelector?: string;
		tabId?: number;
		format?: 'html' | 'text' | 'markdown';
	}): Promise<string> {
		const format = options?.format ?? 'markdown';
		if (options?.cssSelector && format === 'markdown') {
			throw new Error('When using a CSS selector, the `format` option can not be `markdown`.');
		}

		const params: { field: string; selector?: string; tabId?: number } = {
			field: format
		};

		if (options?.cssSelector) {
			params.selector = options.cssSelector;
		}

		if (options?.tabId) {
			params.tabId = options.tabId;
		}

		const result = await sendBrowserRequest<{ value: string }>('getTab', params);
		return result.value;
	}
};
