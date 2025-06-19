import { writeOutput } from '../io';
import * as crypto from 'crypto';
import type * as api from '@raycast/api';

type ClipboardContent = {
	text?: string;
	html?: string;
	file?: string;
};

type ReadResult = {
	text?: string;
	html?: string;
	file?: string;
};

const pendingRequests = new Map<
	string,
	{ resolve: (value: any) => void; reject: (reason?: any) => void }
>();

function sendRequest<T>(type: string, payload: object): Promise<T> {
	return new Promise((resolve, reject) => {
		const requestId = crypto.randomUUID();
		pendingRequests.set(requestId, { resolve, reject });
		writeOutput({
			type,
			payload: { requestId, ...payload }
		});
		setTimeout(() => {
			if (pendingRequests.has(requestId)) {
				pendingRequests.delete(requestId);
				reject(new Error(`Request for ${type} timed out`));
			}
		}, 5000);
	});
}

export function handleClipboardResponse(requestId: string, result: any, error?: string) {
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

function normalizeContent(content: string | number | api.Clipboard.Content): ClipboardContent {
	if (typeof content === 'string' || typeof content === 'number') {
		return { text: String(content) };
	}
	return content;
}

export const Clipboard: typeof api.Clipboard = {
	async copy(content, options) {
		const normalized = normalizeContent(content);
		return sendRequest<void>('clipboard-copy', { content: normalized, options });
	},
	async paste(content) {
		const normalized = normalizeContent(content);
		return sendRequest<void>('clipboard-paste', { content: normalized });
	},
	async clear() {
		return sendRequest<void>('clipboard-clear', {});
	},
	async read(options) {
		return sendRequest<ReadResult>('clipboard-read', { offset: options?.offset });
	},
	async readText(options) {
		const result = await sendRequest<ReadResult>('clipboard-read-text', {
			offset: options?.offset
		});
		return result.text;
	}
};
