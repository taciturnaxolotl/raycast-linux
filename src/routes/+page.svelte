<script lang="ts">
	import { sidecarService } from '$lib/sidecar.svelte';
	import { uiStore } from '$lib/ui.svelte';
	import { untrack } from 'svelte';
	import SettingsView from '$lib/components/SettingsView.svelte';
	import type { PluginInfo } from '@raycast-linux/protocol';
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';
	import CommandPalette from '$lib/components/CommandPalette.svelte';
	import PluginRunner from '$lib/components/PluginRunner.svelte';
	import Extensions from '$lib/components/Extensions.svelte';
	import OAuthView from '$lib/components/OAuthView.svelte';
	import { openUrl } from '@tauri-apps/plugin-opener';
	import ClipboardHistoryView from '$lib/components/ClipboardHistoryView.svelte';
	import QuicklinkForm from '$lib/components/QuicklinkForm.svelte';
	import { quicklinksStore } from '$lib/quicklinks.svelte';

	type ViewState =
		| 'plugin-list'
		| 'plugin-running'
		| 'settings'
		| 'extensions-store'
		| 'clipboard-history'
		| 'create-quicklink';

	type App = { name: string; comment?: string; exec: string; icon_path?: string };

	let viewState = $state<ViewState>('plugin-list');
	let installedApps = $state<App[]>([]);
	let oauthStatus: 'initial' | 'authorizing' | 'success' | 'error' = $state('initial');

	const storePlugin: PluginInfo = {
		title: 'Discover Extensions',
		description: 'Browse and install new extensions from the Store',
		pluginTitle: 'Store',
		pluginName: 'Store',
		commandName: 'index',
		pluginPath: 'builtin:store',
		icon: 'store-16',
		preferences: [],
		mode: 'view'
	};

	const clipboardHistoryPlugin: PluginInfo = {
		title: 'Clipboard History',
		description: 'View, search, and manage your clipboard history',
		pluginTitle: 'Clipboard History',
		pluginName: 'Clipboard History',
		commandName: 'index',
		pluginPath: 'builtin:history',
		icon: 'copy-clipboard-16',
		preferences: [],
		mode: 'view'
	};

	const createQuicklinkPlugin: PluginInfo = {
		title: 'Create Quicklink',
		description: 'Create a new Quicklink',
		pluginTitle: 'Quicklinks',
		pluginName: 'Quicklinks',
		commandName: 'create-quicklink',
		pluginPath: 'builtin:create-quicklink',
		icon: 'link-16',
		preferences: [],
		mode: 'view'
	};

	const { pluginList, currentPreferences } = $derived(uiStore);
	const { quicklinks } = $derived(quicklinksStore);
	const allPlugins = $derived([
		...pluginList,
		storePlugin,
		clipboardHistoryPlugin,
		createQuicklinkPlugin
	]);

	$effect(() => {
		untrack(() => {
			sidecarService.setOnGoBackToPluginList(() => {
				viewState = 'plugin-list';
				uiStore.setCurrentRunningPlugin(null);
			});
			sidecarService.start();
			quicklinksStore.fetchQuicklinks();
			return () => sidecarService.stop();
		});
	});

	$effect(() => {
		invoke('get_installed_apps').then((apps) => {
			installedApps = apps as App[];
		});
	});

	onMount(() => {
		const unlisten = listen<string>('deep-link', (event) => {
			console.log('Received deep link:', event.payload);
			const url = event.payload;
			handleDeepLink(url);
		});

		return () => {
			unlisten.then((fn) => fn());
		};
	});

	function handleDeepLink(url: string) {
		try {
			const urlObj = new URL(url);
			console.log('Processing deep link:', {
				protocol: urlObj.protocol,
				host: urlObj.host,
				pathname: urlObj.pathname,
				search: urlObj.search
			});

			if (urlObj.protocol === 'raycast:') {
				if (urlObj.host === 'oauth' || urlObj.pathname.startsWith('/redirect')) {
					const params = urlObj.searchParams;
					const code = params.get('code');
					const state = params.get('state');

					if (sidecarService.oauthState) {
						oauthStatus = 'success';
						setTimeout(() => {
							sidecarService.oauthState = null;
							oauthStatus = 'initial';
						}, 2000);
					}

					if (code && state) {
						sidecarService.dispatchEvent('oauth-authorize-response', { code, state });
					} else {
						const error = params.get('error') || 'Unknown OAuth error';
						const errorDescription = params.get('error_description');
						sidecarService.dispatchEvent('oauth-authorize-response', {
							state,
							error: `${error}: ${errorDescription}`
						});
					}
				} else {
					switch (urlObj.host) {
						case 'extensions':
							viewState = 'extensions-store';
							break;
						default:
							viewState = 'plugin-list';
					}
				}
			}
		} catch (error) {
			console.error('Error parsing deep link:', error);
			viewState = 'plugin-list';
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (viewState === 'plugin-list' && event.key === ',' && (event.metaKey || event.ctrlKey)) {
			event.preventDefault();
			viewState = 'settings';
			return;
		}
	}

	function handleRunPlugin(plugin: PluginInfo) {
		console.log('handleRunPlugin', plugin);
		if (plugin.pluginPath === 'builtin:store') {
			viewState = 'extensions-store';
			return;
		}
		if (plugin.pluginPath === 'builtin:history') {
			viewState = 'clipboard-history';
			return;
		}
		if (plugin.pluginPath === 'builtin:create-quicklink') {
			viewState = 'create-quicklink';
			return;
		}

		uiStore.setCurrentRunningPlugin(plugin);
		sidecarService.dispatchEvent('run-plugin', {
			pluginPath: plugin.pluginPath,
			commandName: plugin.commandName,
			mode: plugin.mode
		});

		if (plugin.mode !== 'no-view') {
			uiStore.resetForNewPlugin();
			viewState = 'plugin-running';
		}
	}

	function handleBackToPluginList() {
		viewState = 'plugin-list';
	}

	function handleSavePreferences(pluginName: string, values: Record<string, unknown>) {
		sidecarService.setPreferences(pluginName, values);
	}

	function handleGetPreferences(pluginName: string) {
		sidecarService.getPreferences(pluginName);
	}

	function handleDispatch(instanceId: number, handlerName: string, args: unknown[]) {
		sidecarService.dispatchEvent('dispatch-event', { instanceId, handlerName, args });
	}

	function handlePopView() {
		sidecarService.dispatchEvent('pop-view');
	}

	function handleToastAction(toastId: number, actionType: 'primary' | 'secondary') {
		sidecarService.dispatchEvent('dispatch-toast-action', { toastId, actionType });
	}

	function onExtensionInstalled() {
		sidecarService.requestPluginList();
	}

	function handleOauthSignIn() {
		if (sidecarService.oauthState?.url) {
			openUrl(sidecarService.oauthState.url);
			oauthStatus = 'authorizing';
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if sidecarService.oauthState}
	<OAuthView
		providerName={sidecarService.oauthState.providerName}
		providerIcon={sidecarService.oauthState.providerIcon}
		description={sidecarService.oauthState.description}
		authUrl={sidecarService.oauthState.url}
		status={oauthStatus}
		onSignIn={handleOauthSignIn}
		onBack={() => (sidecarService.oauthState = null)}
	/>
{/if}

{#if viewState === 'plugin-list'}
	<CommandPalette plugins={allPlugins} onRunPlugin={handleRunPlugin} {installedApps} {quicklinks} />
{:else if viewState === 'settings'}
	<SettingsView
		plugins={pluginList}
		onBack={handleBackToPluginList}
		onSavePreferences={handleSavePreferences}
		onGetPreferences={handleGetPreferences}
		{currentPreferences}
	/>
{:else if viewState === 'extensions-store'}
	<Extensions onBack={() => (viewState = 'plugin-list')} onInstall={onExtensionInstalled} />
{:else if viewState === 'plugin-running'}
	{#key uiStore.currentRunningPlugin?.pluginPath}
		<PluginRunner
			onDispatch={handleDispatch}
			onPopView={handlePopView}
			onToastAction={handleToastAction}
		/>
	{/key}
{:else if viewState === 'clipboard-history'}
	<ClipboardHistoryView onBack={() => (viewState = 'plugin-list')} />
{:else if viewState === 'create-quicklink'}
	<QuicklinkForm onBack={handleBackToPluginList} onSave={handleBackToPluginList} />
{/if}
