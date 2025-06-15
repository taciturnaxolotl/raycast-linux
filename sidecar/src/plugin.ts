import React from 'react';
import { updateContainer } from './reconciler';
import { writeLog, writeOutput } from './io';
import { getRaycastApi } from './api';
import { inspect } from 'util';
import * as fs from 'fs';
import * as path from 'path';
import type { PluginInfo } from '@raycast-linux/protocol';

const createPluginRequire =
	() =>
	(moduleName: string): unknown => {
		if (moduleName === 'react') {
			return React;
		}

		if (moduleName.startsWith('@raycast/api')) {
			return getRaycastApi();
		}

		try {
			return eval('require')(moduleName);
		} catch (error) {
			writeLog(`Failed to require module: ${moduleName}, error: ${error}`);
			throw error;
		}
	};

export const discoverPlugins = (): PluginInfo[] => {
	const pluginsBaseDir = path.join(
		process.env.HOME || '/tmp',
		'.local/share/raycast-linux/plugins'
	);
	const plugins: PluginInfo[] = [];

	try {
		if (!fs.existsSync(pluginsBaseDir)) {
			writeLog('Plugins directory does not exist, creating it...');
			fs.mkdirSync(pluginsBaseDir, { recursive: true });
			return [];
		}

		const pluginDirs = fs
			.readdirSync(pluginsBaseDir, { withFileTypes: true })
			.filter((dirent) => dirent.isDirectory())
			.map((dirent) => dirent.name);

		for (const pluginDirName of pluginDirs) {
			const pluginDir = path.join(pluginsBaseDir, pluginDirName);
			const packageJsonPath = path.join(pluginDir, 'package.json');

			if (!fs.existsSync(packageJsonPath)) {
				writeLog(`Plugin ${pluginDirName} has no package.json, skipping`);
				continue;
			}

			try {
				const packageJson: {
					name?: string;
					title?: string;
					description?: string;
					icon?: string;
					commands?: Array<{
						name: string;
						title?: string;
						description?: string;
						icon?: string;
						subtitle?: string;
					}>;
				} = JSON.parse(fs.readFileSync(packageJsonPath, 'utf-8'));

				const commands = packageJson.commands || [];

				for (const command of commands) {
					const commandFilePath = path.join(pluginDir, `${command.name}.js`);

					if (fs.existsSync(commandFilePath)) {
						plugins.push({
							title: command.title || command.name,
							description:
								command.description || packageJson.description || `${command.name} command`,
							pluginName: packageJson.name || pluginDirName,
							commandName: command.name,
							pluginPath: commandFilePath,
							icon: command.icon || packageJson.icon
						});
					} else {
						writeLog(`Command file ${commandFilePath} not found for command ${command.name}`);
					}
				}
			} catch (error) {
				writeLog(`Error parsing package.json for plugin ${pluginDirName}: ${error}`);
			}
		}

		writeLog(`Discovered ${plugins.length} commands from ${pluginDirs.length} plugins`);
		return plugins;
	} catch (error) {
		writeLog(`Error discovering plugins: ${error}`);
		return [];
	}
};

export const loadPlugin = (pluginPath: string): string => {
	try {
		if (!fs.existsSync(pluginPath)) {
			throw new Error(`Plugin file not found: ${pluginPath}`);
		}
		return fs.readFileSync(pluginPath, 'utf-8');
	} catch (error) {
		writeLog(`Error loading plugin from ${pluginPath}: ${error}`);
		throw error;
	}
};

export const runPlugin = (pluginPath?: string): void => {
	let scriptText: string;

	if (pluginPath) {
		scriptText = loadPlugin(pluginPath);
	} else {
		const fallbackPluginsDir = path.join(
			process.env.HOME || '/tmp',
			'.local/share/raycast-linux/plugins'
		);
		const fallbackPath = path.join(fallbackPluginsDir, 'google-translate', 'translate-form.js');

		if (fs.existsSync(fallbackPath)) {
			scriptText = loadPlugin(fallbackPath);
		} else {
			const oldFallbackPath = path.join(__dirname, '../dist/plugin/translate-form.txt');
			if (fs.existsSync(oldFallbackPath)) {
				scriptText = loadPlugin(oldFallbackPath);
			} else {
				throw new Error('No plugin specified and no fallback plugin found');
			}
		}
	}

	const pluginModule = {
		exports: {} as { default: React.ComponentType | null }
	};

	const scriptFunction = new Function(
		'require',
		'module',
		'exports',
		'React',
		'console',
		scriptText
	);

	const mockConsole = {
		log: (...args: unknown[]) => {
			writeLog('[plugin] log: ' + args.map((arg) => inspect(arg, { depth: null })).join(' '));
		},
		warn: (...args: unknown[]) => {
			writeLog('[plugin] warn: ' + args.map((arg) => inspect(arg, { depth: null })).join(' '));
		},
		error: (...args: unknown[]) => {
			writeLog('[plugin] error: ' + args.map((arg) => inspect(arg, { depth: null })).join(' '));
		}
	};

	scriptFunction(createPluginRequire(), pluginModule, pluginModule.exports, React, mockConsole);

	const PluginRootComponent = pluginModule.exports.default;

	if (!PluginRootComponent) {
		throw new Error('Plugin did not export a default component.');
	}

	writeLog('Plugin loaded. Initializing React render...');
	const AppElement = React.createElement(PluginRootComponent);
	updateContainer(AppElement, () => {
		writeLog('Initial render complete');
	});
};

export const sendPluginList = (): void => {
	const plugins = discoverPlugins();
	writeOutput({
		type: 'plugin-list',
		payload: plugins
	});
};
