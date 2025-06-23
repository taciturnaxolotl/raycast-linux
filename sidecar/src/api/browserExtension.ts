import { writeOutput } from '../io';
import * as crypto from 'crypto';

const pendingRequests = new Map<
	string,
	{ resolve: (value: unknown) => void; reject: (reason?: unknown) => void }
>();

export function handleBrowserExtensionResponse(requestId: string, result: unknown, error?: string) {
	const promise = pendingRequests.get(requestId);
	if (promise) {
		if (error) {
			promise.reject(new Error(error));
		} else {
			promise.resolve(result);
		}
		pendingRequests.delete(requestId);
	}
}

function sendRequest<T>(method: string, params: unknown): Promise<T> {
	return new Promise((resolve, reject) => {
		const requestId = crypto.randomUUID();
		pendingRequests.set(requestId, { resolve: resolve as (value: unknown) => void, reject });
		writeOutput({
			type: 'browser-extension-request',
			payload: { requestId, method, params }
		});
		setTimeout(() => {
			if (pendingRequests.has(requestId)) {
				pendingRequests.delete(requestId);
				reject(new Error(`Request for ${method} timed out`));
			}
		}, 5000);
	});
}

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

export const BrowserExtensionAPI = {
	async getTabs(): Promise<Tab[]> {
		const result = await sendRequest<{ value: RawTab[] }>('getTabs', {});
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

		const result = await sendRequest<{ value: string }>('getTab', params);
		return result.value;
	}
};
