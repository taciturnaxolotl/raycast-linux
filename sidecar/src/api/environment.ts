import { LaunchType } from './types';
import * as fs from 'fs';
import { writeOutput } from '../io';
import type { Application } from './types';
import { config } from '../config';
import { browserExtensionState } from '../state';
import * as crypto from 'crypto';

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

const pendingSystemRequests = new Map<
	string,
	{ resolve: (value: unknown) => void; reject: (reason?: unknown) => void }
>();

function sendSystemRequest<T>(type: string, payload: object = {}): Promise<T> {
	return new Promise((resolve, reject) => {
		const requestId = crypto.randomUUID();
		pendingSystemRequests.set(requestId, { resolve, reject });
		writeOutput({
			type: `system-${type}`,
			payload: { requestId, ...payload }
		});
		setTimeout(() => {
			if (pendingSystemRequests.has(requestId)) {
				pendingSystemRequests.delete(requestId);
				reject(new Error(`Request for ${type} timed out`));
			}
		}, 5000); // 5-second timeout
	});
}

export function handleSystemResponse(requestId: string, result: unknown, error?: string) {
	const promise = pendingSystemRequests.get(requestId);
	if (promise) {
		if (error) {
			promise.reject(new Error(error));
		} else {
			promise.resolve(result);
		}
		pendingSystemRequests.delete(requestId);
	}
}

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

export async function getSelectedFinderItems(): Promise<FileSystemItem[]> {
	return sendSystemRequest<FileSystemItem[]>('get-selected-finder-items');
}

export async function getSelectedText(): Promise<string> {
	return sendSystemRequest<string>('get-selected-text');
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

export async function getApplications(path?: fs.PathLike): Promise<Application[]> {
	const pathString = path ? path.toString() : undefined;
	return sendSystemRequest<Application[]>('get-applications', { path: pathString });
}

export async function getDefaultApplication(path: fs.PathLike): Promise<Application> {
	return sendSystemRequest<Application>('get-default-application', { path: path.toString() });
}

export async function getFrontmostApplication(): Promise<Application> {
	return sendSystemRequest<Application>('get-frontmost-application');
}

export async function showInFinder(path: fs.PathLike): Promise<void> {
	return sendSystemRequest<void>('show-in-finder', { path: path.toString() });
}

export async function trash(path: fs.PathLike | fs.PathLike[]): Promise<void> {
	const paths = (Array.isArray(path) ? path : [path]).map((p) => p.toString());
	return sendSystemRequest<void>('trash', { paths });
}
