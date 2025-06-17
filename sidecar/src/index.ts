import { createInterface } from 'readline';
import { writeLog, writeOutput } from './io';
import { runPlugin, sendPluginList } from './plugin';
import { instances, navigationStack } from './state';
import { batchedUpdates, updateContainer } from './reconciler';
import { preferencesStore } from './preferences';
import type { RaycastInstance } from './types';
import {
	handleGetSelectedFinderItemsResponse,
	handleSelectedTextResponse,
	type FileSystemItem
} from './api/environment';

process.on('unhandledRejection', (reason: unknown) => {
	writeLog(`--- UNHANDLED PROMISE REJECTION ---`);
	const stack = reason && typeof reason === 'object' && 'stack' in reason ? reason.stack : reason;
	writeLog(stack);
});

const rl = createInterface({ input: process.stdin });

rl.on('line', (line) => {
	batchedUpdates(() => {
		try {
			const command: { action: string; payload: unknown } = JSON.parse(line);

			switch (command.action) {
				case 'request-plugin-list':
					sendPluginList();
					break;
				case 'run-plugin': {
					const { pluginPath, mode } = command.payload as {
						pluginPath?: string;
						commandName?: string;
						mode?: 'view' | 'no-view';
					};
					runPlugin(pluginPath, mode);
					break;
				}
				case 'get-preferences': {
					const { pluginName } = command.payload as { pluginName: string };
					const preferences = preferencesStore.getAllPreferences();
					writeOutput({
						type: 'preference-values',
						payload: {
							pluginName,
							values: preferences[pluginName] || {}
						}
					});
					break;
				}
				case 'set-preferences': {
					const { pluginName, values } = command.payload as {
						pluginName: string;
						values: Record<string, unknown>;
					};
					preferencesStore.setPreferenceValues(pluginName, values);
					break;
				}
				case 'pop-view': {
					const previousElement = navigationStack.pop();
					if (previousElement) {
						updateContainer(previousElement);
					} else {
						writeOutput({ type: 'go-back-to-plugin-list', payload: {} });
					}
					break;
				}
				case 'dispatch-event': {
					const { instanceId, handlerName, args } = command.payload as {
						instanceId: number;
						handlerName: string;
						args: unknown[];
					};

					const instance = instances.get(instanceId);
					if (!instance) {
						writeLog(`Instance ${instanceId} not found.`);
						return;
					}

					if (!('props' in instance)) {
						return;
					}

					const raycastInstance = instance as RaycastInstance;

					const props = raycastInstance._unserializedProps;
					const handler = props?.[handlerName];

					if (typeof handler === 'function') {
						handler(...args);
					} else {
						writeLog(
							`Handler ${handlerName} not found or not a function on instance ${instanceId}`
						);
					}
					break;
				}
				case 'selected-text-response': {
					const { requestId, text, error } = command.payload as {
						requestId: string;
						text?: string | null;
						error?: string;
					};
					handleSelectedTextResponse(requestId, text ?? null, error);
					break;
				}
				case 'selected-finder-items-response': {
					const { requestId, items, error } = command.payload as {
						requestId: string;
						items?: FileSystemItem[] | null;
						error?: string;
					};
					handleGetSelectedFinderItemsResponse(requestId, items ?? null, error);
					break;
				}
				default:
					writeLog(`Unknown command action: ${command.action}`);
			}
		} catch (err: unknown) {
			const error =
				err instanceof Error
					? { message: err.message, stack: err.stack }
					: { message: String(err) };
			writeLog(`ERROR: ${error.message} \n ${error.stack ?? ''}`);
			writeOutput({ type: 'error', payload: error.message });
		}
	});
});

writeLog('Node.js Sidecar started successfully with React Reconciler.');
sendPluginList();
