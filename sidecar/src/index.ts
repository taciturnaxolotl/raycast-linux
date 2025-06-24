import { createInterface } from 'readline';
import { writeLog, writeOutput } from './io';
import { runPlugin, sendPluginList } from './plugin';
import { instances, navigationStack, toasts, browserExtensionState } from './state';
import { batchedUpdates, updateContainer } from './reconciler';
import { preferencesStore } from './preferences';
import type { RaycastInstance } from './types';
import { handleSystemResponse } from './api/environment';
import { handleBrowserExtensionResponse } from './api/browserExtension';
import { handleClipboardResponse } from './api/clipboard';
import { handleOAuthResponse, handleTokenResponse } from './api/oauth';

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

			if (command.action.startsWith('system-') && command.action.endsWith('-response')) {
				const { requestId, result, error } = command.payload as {
					requestId: string;
					result?: unknown;
					error?: string;
				};
				handleSystemResponse(requestId, result, error);
				return;
			}

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
				case 'dispatch-toast-action': {
					const { toastId, actionType } = command.payload as {
						toastId: number;
						actionType: 'primary' | 'secondary';
					};
					const toast = toasts.get(toastId);
					if (toast) {
						const action = actionType === 'primary' ? toast.primaryAction : toast.secondaryAction;
						if (action?.onAction) {
							action.onAction(toast);
						}
					}
					break;
				}
				case 'trigger-toast-hide': {
					const { toastId } = command.payload as { toastId: number };
					const toast = toasts.get(toastId);
					toast?.hide();
					break;
				}
				case 'browser-extension-response': {
					const { requestId, result, error } = command.payload as {
						requestId: string;
						result?: unknown;
						error?: string;
					};
					handleBrowserExtensionResponse(requestId, result, error);
					break;
				}
				case 'browser-extension-connection-status': {
					const { isConnected } = command.payload as { isConnected: boolean };
					browserExtensionState.isConnected = isConnected;
					break;
				}
				case 'oauth-authorize-response': {
					const { code, state, error } = command.payload as {
						code: string;
						state: string;
						error?: string;
					};
					handleOAuthResponse(state, code, state, error);
					break;
				}
				case 'oauth-get-tokens-response':
				case 'oauth-set-tokens-response':
				case 'oauth-remove-tokens-response': {
					const { requestId, result, error } = command.payload as {
						requestId: string;
						result?: unknown;
						error?: string;
					};
					handleTokenResponse(requestId, result, error);
					break;
				}
				case 'clipboard-read-text-response':
				case 'clipboard-read-response':
				case 'clipboard-copy-response':
				case 'clipboard-paste-response':
				case 'clipboard-clear-response': {
					const { requestId, result, error } = command.payload as {
						requestId: string;
						result?: unknown;
						error?: string;
					};
					handleClipboardResponse(requestId, result, error);
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

			throw err;
		}
	});
});

writeLog('Node.js Sidecar started successfully with React Reconciler.');
sendPluginList();
