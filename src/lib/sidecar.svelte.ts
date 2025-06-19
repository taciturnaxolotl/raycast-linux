import { Command, type Child, open as shellOpen } from '@tauri-apps/plugin-shell';
import { Unpackr } from 'msgpackr';
import { uiStore } from '$lib/ui.svelte';
import { SidecarMessageWithPluginsSchema } from '@raycast-linux/protocol';
import { invoke } from '@tauri-apps/api/core';
import { appCacheDir, appLocalDataDir } from '@tauri-apps/api/path';

class SidecarService {
	#sidecarChild: Child | null = $state(null);
	#receiveBuffer = Buffer.alloc(0);
	#unpackr = new Unpackr();
	#onGoBackToPluginList: (() => void) | null = null;
	#browserExtensionConnectionInterval: ReturnType<typeof setInterval> | null = null;

	logs: string[] = $state([]);

	constructor() {}

	get isRunning() {
		return this.#sidecarChild !== null;
	}

	setOnGoBackToPluginList(callback: () => void) {
		this.#onGoBackToPluginList = callback;
	}

	start = async () => {
		if (this.#sidecarChild) {
			this.#log('Sidecar service is already running.');
			return;
		}

		this.#log('Starting sidecar service...');
		try {
			const args: string[] = [];
			args.push(`--data-dir=${await appLocalDataDir()}`);
			args.push(`--cache-dir=${await appCacheDir()}`);
			console.log(args);
			const command = Command.sidecar('binaries/app', args.length > 0 ? args : undefined, {
				encoding: 'raw'
			});

			command.stdout.on('data', this.#handleStdout);
			command.stderr.on('data', (line) => this.#log(`STDERR: ${line}`));

			this.#sidecarChild = await command.spawn();
			this.#log(`Sidecar spawned with PID: ${this.#sidecarChild.pid}`);

			this.requestPluginList();

			this.#browserExtensionConnectionInterval = setInterval(async () => {
				try {
					const isConnected = await invoke<boolean>('browser_extension_check_connection');
					this.dispatchEvent('browser-extension-connection-status', { isConnected });
				} catch (e) {
					this.#log(`Error checking browser extension connection: ${e}`);
					this.dispatchEvent('browser-extension-connection-status', { isConnected: false });
				}
			}, 5000);
		} catch (e) {
			this.#log(`ERROR starting sidecar: ${e}`);
			console.error('Failed to start sidecar:', e);
		}
	};

	stop = () => {
		if (this.#sidecarChild) {
			this.#log('Stopping sidecar service...');
			this.#sidecarChild.kill();
			this.#sidecarChild = null;
		}
		if (this.#browserExtensionConnectionInterval) {
			clearInterval(this.#browserExtensionConnectionInterval);
			this.#browserExtensionConnectionInterval = null;
		}
	};

	dispatchEvent = (action: string, payload?: object) => {
		if (!this.#sidecarChild) {
			this.#log('Cannot dispatch event, sidecar is not running.');
			return;
		}
		const message = JSON.stringify({ action, payload });
		this.#sidecarChild.write(message + '\n');
	};

	requestPluginList = () => {
		this.dispatchEvent('request-plugin-list');
	};

	getPreferences = (pluginName: string) => {
		this.dispatchEvent('get-preferences', { pluginName });
	};

	setPreferences = (pluginName: string, values: Record<string, unknown>) => {
		this.dispatchEvent('set-preferences', { pluginName, values });
	};

	#handleStdout = (chunk: Uint8Array) => {
		try {
			this.#receiveBuffer = Buffer.concat([this.#receiveBuffer, Buffer.from(chunk)]);
			this.#processReceiveBuffer();
		} catch (e) {
			this.#log(`ERROR processing stdout: ${e}`);
			console.error('Failed to parse sidecar message:', chunk, e);
		}
	};

	#processReceiveBuffer = () => {
		while (this.#receiveBuffer.length >= 4) {
			const messageLength = this.#receiveBuffer.readUInt32BE(0);
			const totalLength = 4 + messageLength;

			if (this.#receiveBuffer.length >= totalLength) {
				const messagePayload = this.#receiveBuffer.subarray(4, totalLength);
				this.#receiveBuffer = this.#receiveBuffer.subarray(totalLength);

				try {
					const message = this.#unpackr.unpack(messagePayload);
					this.#routeMessage(message);
				} catch (e) {
					console.error('Failed to unpack sidecar message:', e);
					this.#log(`ERROR unpacking message: ${e}`);
				}
			} else {
				break;
			}
		}
	};

	#routeMessage = (message: unknown) => {
		const result = SidecarMessageWithPluginsSchema.safeParse(message);

		if (!result.success) {
			this.#log(`ERROR: Received invalid message from sidecar: ${result.error.message}`);
			console.error('Invalid sidecar message:', result.error);
			return;
		}

		const typedMessage = result.data;

		if (typedMessage.type === 'log') {
			this.#log(`SIDECAR: ${typedMessage.payload}`);
			return;
		}

		if (typedMessage.type === 'open') {
			const { target, application } = typedMessage.payload;
			shellOpen(target, application).catch((err) => {
				this.#log(`ERROR: Failed to open '${target}': ${err}`);
				console.error(`Failed to open '${target}':`, err);
			});
			return;
		}

		if (typedMessage.type === 'plugin-list') {
			uiStore.setPluginList(typedMessage.payload);
			return;
		}

		if (typedMessage.type === 'preference-values') {
			uiStore.setCurrentPreferences(typedMessage.payload.values);
			return;
		}

		if (typedMessage.type === 'go-back-to-plugin-list') {
			if (this.#onGoBackToPluginList) {
				this.#onGoBackToPluginList();
			}
			return;
		}

		if (typedMessage.type === 'get-selected-text') {
			const { requestId } = typedMessage.payload;
			invoke('get_selected_text')
				.then((text) => {
					this.dispatchEvent('selected-text-response', {
						requestId,
						text
					});
				})
				.catch((error) => {
					this.#log(`ERROR getting selected text: ${error}`);
					this.dispatchEvent('selected-text-response', {
						requestId,
						error: String(error)
					});
				});
			return;
		}

		if (typedMessage.type === 'get-selected-finder-items') {
			const { requestId } = typedMessage.payload;
			invoke('get_selected_finder_items')
				.then((items) => {
					this.dispatchEvent('selected-finder-items-response', {
						requestId,
						items
					});
				})
				.catch((error) => {
					this.#log(`ERROR getting selected finder items: ${error}`);
					this.dispatchEvent('selected-finder-items-response', {
						requestId,
						error: String(error)
					});
				});
			return;
		}

		if (typedMessage.type === 'browser-extension-request') {
			const { requestId, method, params } = typedMessage.payload;
			invoke('browser_extension_request', { method, params })
				.then((result) => {
					this.dispatchEvent('browser-extension-response', { requestId, result });
				})
				.catch((error) => {
					this.#log(`ERROR from browser extension request: ${error}`);
					this.dispatchEvent('browser-extension-response', { requestId, error: String(error) });
				});
			return;
		}

		const commands = typedMessage.type === 'BATCH_UPDATE' ? typedMessage.payload : [typedMessage];
		if (commands.length > 0) {
			uiStore.applyCommands(commands);
		}
	};

	#log = (message: string) => {
		console.log(`[SidecarService] ${message}`);
		this.logs.push(message);
	};
}

export const sidecarService = new SidecarService();
