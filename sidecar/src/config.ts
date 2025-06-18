import * as path from 'path';

export interface SidecarConfig {
	dataDir: string;
	cacheDir: string;
	supportDir: string;
	pluginsDir: string;
	preferencesFile: string;
	assetsDir: string;
}

function getArg(name: string): string | undefined {
	const prefix = `--${name}=`;
	for (const arg of process.argv.slice(2)) {
		if (arg.startsWith(prefix)) {
			return arg.slice(prefix.length);
		}
	}
	const key = `--${name}`;
	const idx = process.argv.indexOf(key);
	if (idx !== -1 && idx + 1 < process.argv.length) {
		return process.argv[idx + 1];
	}
	return undefined;
}

export function createConfig(): SidecarConfig {
	const dataDir = getArg('data-dir');
	const cacheBase = getArg('cache-dir');

	if (!dataDir || !cacheBase) {
		throw new Error('data-dir and cache-dir are required');
	}

	const pluginsDir = path.join(dataDir, 'plugins');
	return {
		dataDir,
		cacheDir: path.join(cacheBase, 'sidecar'),
		supportDir: path.join(dataDir, 'support'),
		pluginsDir,
		preferencesFile: path.join(dataDir, 'preferences.json'),
		assetsDir: pluginsDir
	};
}

export const config = createConfig();
