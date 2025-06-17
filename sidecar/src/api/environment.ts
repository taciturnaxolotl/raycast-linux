import { LaunchType } from './types';
import * as fs from 'fs';
import { writeOutput } from '../io';
import type { Application } from './types';

const supportPath = '/home/byte/code/raycast-linux/sidecar/dist/plugin/support/';
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

export const environment = {
	appearance: 'dark' as const,
	assetsPath: '/home/byte/code/raycast-linux/sidecar/dist/plugin/assets/',
	commandMode: 'view' as const,
	commandName: 'index',
	extensionName: 'my-extension',
	isDevelopment: true,
	launchType: LaunchType.UserInitiated,
	ownerOrAuthorName: 'Raycast',
	raycastVersion: '1.0.0',
	supportPath: supportPath,
	textSize: 'medium' as const,
	canAccess: (api: unknown): boolean => {
		return true;
	}
};

export async function getSelectedFinderItems(): Promise<FileSystemItem[]> {
	return Promise.reject(new Error('Finder is not the frontmost application.'));
}

export async function getSelectedText(): Promise<string> {
	return Promise.reject(new Error('No text selected in the frontmost application.'));
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
