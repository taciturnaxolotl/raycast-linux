import { LaunchType } from './types';
import * as fs from 'fs';
import { writeOutput } from '../io';
import type { Application } from './types';
import { config } from '../config';
import { browserExtensionState } from '../state';

const supportPath = config.supportDir;
try {
	if (!fs.existsSync(supportPath)) {
		fs.mkdirSync(supportPath, { recursive: true });
	}
} catch (e) {
	console.error('Could not create support path', e);
}

export interface FileSystemItem {
	path: string;
}

export const BrowserExtension = { name: 'BrowserExtension' };

export const environment = {
	appearance: 'dark' as const,
	assetsPath: config.assetsDir,
	commandMode: 'view' as const,
	commandName: 'index',
	extensionName: 'my-extension',
	isDevelopment: true,
	launchType: LaunchType.UserInitiated,
	ownerOrAuthorName: 'Raycast',
	raycastVersion: '1.0.0',
	supportPath: supportPath,
	textSize: 'medium' as const,
	canAccess: (feature: { name: string }): boolean => {
		if (feature && feature.name === 'BrowserExtension') {
			return browserExtensionState.isConnected;
		}
		return true;
	}
};

const pendingFinderItemsRequests = new Map<
	string,
	{ resolve: (items: FileSystemItem[]) => void; reject: (error: Error) => void }
>();

export async function getSelectedFinderItems(): Promise<FileSystemItem[]> {
	return new Promise((resolve, reject) => {
		const requestId = Math.random().toString(36).substring(7);

		pendingFinderItemsRequests.set(requestId, { resolve, reject });

		writeOutput({
			type: 'get-selected-finder-items',
			payload: { requestId }
		});

		setTimeout(() => {
			if (pendingFinderItemsRequests.has(requestId)) {
				pendingFinderItemsRequests.delete(requestId);
				reject(new Error('Timeout: Could not get selected finder items'));
			}
		}, 1000);
	});
}

export function handleGetSelectedFinderItemsResponse(
	requestId: string,
	items: FileSystemItem[] | null,
	error?: string
) {
	const pending = pendingFinderItemsRequests.get(requestId);
	if (pending) {
		pendingFinderItemsRequests.delete(requestId);
		if (error) {
			pending.reject(new Error(error));
		} else {
			pending.resolve(items || []);
		}
	}
}

const pendingTextRequests = new Map<
	string,
	{ resolve: (text: string) => void; reject: (error: Error) => void }
>();

export async function getSelectedText(): Promise<string> {
	return new Promise((resolve, reject) => {
		const requestId = Math.random().toString(36).substring(7);

		pendingTextRequests.set(requestId, { resolve, reject });

		writeOutput({
			type: 'get-selected-text',
			payload: {
				requestId
			}
		});

		setTimeout(() => {
			if (pendingTextRequests.has(requestId)) {
				pendingTextRequests.delete(requestId);
				reject(new Error('Timeout: Could not get selected text'));
			}
		}, 1000);
	});
}

export function handleSelectedTextResponse(requestId: string, text: string | null, error?: string) {
	const pending = pendingTextRequests.get(requestId);
	if (pending) {
		pendingTextRequests.delete(requestId);
		if (error) {
			pending.reject(new Error(error));
		} else {
			pending.resolve(text || '');
		}
	}
}

export async function open(target: string, application?: Application | string): Promise<void> {
	let openWith: string | undefined;

	if (typeof application === 'string') {
		openWith = application;
	} else if (application) {
		openWith = application.path;
	}

	writeOutput({
		type: 'open',
		payload: {
			target,
			application: openWith
		}
	});
}
