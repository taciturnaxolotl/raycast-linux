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

	type ViewState = 'plugin-list' | 'plugin-running' | 'settings' | 'extensions-store';

	let viewState = $state<ViewState>('plugin-list');
	let installedApps = $state<any[]>([]);
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

	const { pluginList, currentPreferences } = $derived(uiStore);
	const allPlugins = $derived([...pluginList, storePlugin]);

	$effect(() => {
		untrack(() => {
			sidecarService.setOnGoBackToPluginList(() => {
				viewState = 'plugin-list';
				uiStore.setCurrentRunningPlugin(null);
			});
			sidecarService.start();
			return () => sidecarService.stop();
		});
	});

	$effect(() => {
		invoke('get_installed_apps').then((apps) => {
			installedApps = apps as any[];
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
				if (urlObj.host === 'oauth-callback' || urlObj.pathname.startsWith('/redirect')) {
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
		if (plugin.pluginPath === 'builtin:store') {
			viewState = 'extensions-store';
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

	function handleDispatch(instanceId: number, handlerName: string, args: any[]) {
		sidecarService.dispatchEvent('dispatch-event', { instanceId, handlerName, args });
	}

	function handlePopView() {
		sidecarService.dispatchEvent('pop-view');
	}

	function handleHideToast(toastId: number) {
		sidecarService.dispatchEvent('trigger-toast-hide', { toastId });
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
	<CommandPalette plugins={allPlugins} onRunPlugin={handleRunPlugin} {installedApps} />
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
	<PluginRunner
		onDispatch={handleDispatch}
		onPopView={handlePopView}
		onToastAction={handleToastAction}
		onHideToast={handleHideToast}
	/>
{/if}
