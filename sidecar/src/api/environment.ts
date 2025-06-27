import { LaunchType } from './types';
import * as fs from 'fs';
import { writeOutput } from '../io';
import type { Application } from './types';
import { config } from '../config';
import { browserExtensionState } from '../state';
import { sendRequest } from './rpc';

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
export const AI = { name: 'AI' };

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
		if (feature && feature.name === 'AI') {
			return true;
		}
		return true;
	}
};

export async function getSelectedFinderItems(): Promise<FileSystemItem[]> {
	return sendRequest<FileSystemItem[]>('system-get-selected-finder-items');
}

export async function getSelectedText(): Promise<string> {
	return sendRequest<string>('system-get-selected-text');
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
	return sendRequest<Application[]>('system-get-applications', { path: pathString });
}

export async function getDefaultApplication(path: fs.PathLike): Promise<Application> {
	return sendRequest<Application>('system-get-default-application', { path: path.toString() });
}

export async function getFrontmostApplication(): Promise<Application> {
	return sendRequest<Application>('system-get-frontmost-application');
}

export async function showInFinder(path: fs.PathLike): Promise<void> {
	return sendRequest<void>('system-show-in-finder', { path: path.toString() });
}

export async function trash(path: fs.PathLike | fs.PathLike[]): Promise<void> {
	const paths = (Array.isArray(path) ? path : [path]).map((p) => p.toString());
	return sendRequest<void>('system-trash', { paths });
}
