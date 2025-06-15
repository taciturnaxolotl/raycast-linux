import React from 'react';
import plugin from '../dist/plugin/translate.txt';
import { updateContainer } from './reconciler';
import { writeLog } from './io';
import { getRaycastApi } from './api';
import { inspect } from 'util';

const createPluginRequire =
	() =>
	(moduleName: string): unknown => {
		if (moduleName === 'react') {
			return React;
		}

		if (moduleName.startsWith('@raycast/api')) {
			return getRaycastApi();
		}

		return require(moduleName);
	};

export const runPlugin = (): void => {
	const scriptText = plugin;
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
