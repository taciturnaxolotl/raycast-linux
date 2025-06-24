import { sendRequest } from './rpc';
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
