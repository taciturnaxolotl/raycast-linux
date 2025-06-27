import React from 'react';
import { updateContainer } from './reconciler';
import { writeLog, writeOutput } from './io';
import { getRaycastApi, setCurrentPlugin } from './api';
import { inspect } from 'util';
import * as fs from 'fs';
import * as path from 'path';
import type { PluginInfo, Preference } from '@raycast-linux/protocol';
import { environment } from './api/environment';
import { config } from './config';
import * as ReactJsxRuntime from 'react/jsx-runtime';

const createPluginRequire =
	() =>
	(moduleName: string): unknown => {
		if (moduleName === 'react') {
			return React;
		}

		if (moduleName.startsWith('@raycast/api')) {
			return getRaycastApi();
		}

		if (moduleName === 'react') {
			return React;
		}

		if (moduleName === 'react/jsx-runtime') {
			return ReactJsxRuntime;
		}

		return require(moduleName);
	};

export const discoverPlugins = (): PluginInfo[] => {
	const pluginsBaseDir = config.pluginsDir;
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
					author?: string | { name: string };
					owner?: string;
					commands?: Array<{
						name: string;
						title?: string;
						description?: string;
						icon?: string;
						subtitle?: string;
						mode?: 'view' | 'no-view';
						preferences?: Preference[];
					}>;
					preferences?: Preference[];
				} = JSON.parse(fs.readFileSync(packageJsonPath, 'utf-8'));

				const commands = packageJson.commands || [];

				for (const command of commands) {
					const commandFilePath = path.join(pluginDir, `${command.name}.js`);

					if (fs.existsSync(commandFilePath)) {
						plugins.push({
							title: command.title || command.name,
							description:
								command.description || packageJson.description || `${command.name} command`,
							pluginTitle: packageJson.title || pluginDirName,
							pluginName: packageJson.name || pluginDirName,
							commandName: command.name,
							pluginPath: commandFilePath,
							icon: command.icon || packageJson.icon,
							preferences: packageJson.preferences,
							commandPreferences: command.preferences,
							mode: command.mode,
							author: packageJson.author,
							owner: packageJson.owner
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

interface LaunchProps {
	arguments: Record<string, unknown>;
	launchType: typeof environment.launchType;
}

export const runPlugin = (pluginPath?: string, mode: 'view' | 'no-view' = 'view'): void => {
	let pluginName = 'unknown';
	let preferences: Preference[] = [];

	if (!pluginPath) {
		throw new Error('No plugin specified.');
	}

	const scriptText = loadPlugin(pluginPath);

	const pluginDir = path.dirname(pluginPath);
	const packageJsonPath = path.join(pluginDir, 'package.json');

	if (fs.existsSync(packageJsonPath)) {
		try {
			const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf-8'));
			pluginName = packageJson.name || path.basename(pluginDir);
			const pluginPreferences = packageJson.preferences || [];
			const allCommandPreferences = (packageJson.commands || []).flatMap(
				(cmd: { preferences?: Preference[] }) => cmd.preferences || []
			);
			preferences = [...pluginPreferences, ...allCommandPreferences];
		} catch (error) {
			writeLog(`Error reading plugin package.json: ${error}`);
		}
	}

	environment.assetsPath = path.join(config.pluginsDir, pluginName, 'assets');
	environment.extensionName = pluginName;

	setCurrentPlugin(pluginName, preferences);

	const pluginModule = {
		exports: {} as {
			default: React.ComponentType<LaunchProps> | ((props: LaunchProps) => Promise<void>) | null;
		}
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

	const PluginRoot = pluginModule.exports.default;

	if (!PluginRoot) {
		throw new Error('Plugin did not export a default component.');
	}

	const launchProps: LaunchProps = {
		arguments: {},
		launchType: environment.launchType
	};

	if (mode === 'no-view') {
		if (typeof PluginRoot === 'function') {
			(PluginRoot as (props: LaunchProps) => Promise<void>)(launchProps)
				.then(() => {
					writeLog('No-view command finished.');
					writeOutput({ type: 'go-back-to-plugin-list', payload: {} });
				})
				.catch((e) => {
					writeLog(`No-view command failed: ${e}`);
					writeOutput({ type: 'go-back-to-plugin-list', payload: {} });
				});
		} else {
			throw new Error('No-view command did not export a default function.');
		}
	} else {
		writeLog('Plugin loaded. Initializing React render...');
		const AppElement = React.createElement(PluginRoot as React.ComponentType, launchProps);
		updateContainer(AppElement, () => {
			writeLog('Initial render complete');
		});
	}
};

export const sendPluginList = (): void => {
	const plugins = discoverPlugins();
	writeOutput({
		type: 'plugin-list',
		payload: plugins
	});
};
