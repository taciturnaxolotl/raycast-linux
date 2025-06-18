<script lang="ts">
	import { sidecarService } from '$lib/sidecar.svelte';
	import { uiStore } from '$lib/ui.svelte';
	import type { UINode } from '$lib/types';
	import { untrack, setContext } from 'svelte';
	import MainLayout from '$lib/components/layout/MainLayout.svelte';
	import Header from '$lib/components/layout/Header.svelte';
	import Content from '$lib/components/layout/Content.svelte';
	import Footer from '$lib/components/layout/Footer.svelte';
	import SettingsView from '$lib/components/SettingsView.svelte';
	import type { PluginInfo } from '@raycast-linux/protocol';
	import { invoke } from '@tauri-apps/api/core';
	import CommandPalette from '$lib/components/CommandPalette.svelte';
	import { shortcutToText } from '$lib/renderKey';
	import type { KeyboardShortcut } from '$lib/props';
	import path from 'path';

	type ViewState = 'plugin-list' | 'plugin-running' | 'settings';

	let viewState = $state<ViewState>('plugin-list');
	let installedApps = $state<any[]>([]);

	const {
		uiTree,
		rootNodeId,
		selectedNodeId,
		pluginList,
		currentPreferences,
		toasts,
		currentRunningPlugin,
		primaryAction,
		secondaryAction,
		actionPanel,
		allActions
	} = $derived(uiStore);

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

	const rootNode = $derived(uiTree.get(rootNodeId!));
	const selectedItemNode = $derived(uiTree.get(selectedNodeId!));
	let searchText = $state('');
	const navigationTitle = $derived(rootNode?.props.navigationTitle as string | undefined);

	const assetsPath = $derived(
		currentRunningPlugin ? path.dirname(currentRunningPlugin.pluginPath) + '/assets' : ''
	);
	setContext('assetsPath', assetsPath);

	function handleDispatch(instanceId: number, handlerName: string, args: any[]) {
		sidecarService.dispatchEvent('dispatch-event', { instanceId, handlerName, args });
	}

	function handleSelect(nodeId: number | undefined) {
		uiStore.selectedNodeId = nodeId;
	}

	function handlePopView() {
		sidecarService.dispatchEvent('pop-view');
	}

	function handleKeydown(event: KeyboardEvent) {
		if (viewState === 'plugin-list' && event.key === ',' && (event.metaKey || event.ctrlKey)) {
			event.preventDefault();
			viewState = 'settings';
			return;
		}

		if (viewState === 'plugin-running' && event.key === 'Escape') {
			handlePopView();
			return;
		}
	}

	$effect(() => {
		if (viewState === 'plugin-running' && rootNode) {
			handleDispatch(rootNode.id, 'onSearchTextChange', [searchText]);
		}
	});

	function handleRunPlugin(plugin: PluginInfo) {
		uiStore.setCurrentRunningPlugin(plugin);
		sidecarService.dispatchEvent('run-plugin', {
			pluginPath: plugin.pluginPath,
			commandName: plugin.commandName,
			mode: plugin.mode
		});

		if (plugin.mode !== 'no-view') {
			uiStore.resetForNewPlugin();
			viewState = 'plugin-running';
			searchText = '';
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

	invoke('get_installed_apps').then((apps) => {
		console.log(apps);
		installedApps = apps as any[];
	});

	function handleHideToast(toastId: number) {
		sidecarService.dispatchEvent('trigger-toast-hide', { toastId });
	}

	function handleToastAction(toastId: number, actionType: 'primary' | 'secondary') {
		sidecarService.dispatchEvent('dispatch-toast-action', { toastId, actionType });
	}

	function formatShortcut(shortcut: KeyboardShortcut) {
		return shortcutToText(shortcut);
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if viewState === 'plugin-list'}
	<CommandPalette plugins={pluginList} onRunPlugin={handleRunPlugin} {installedApps} />
{:else if viewState === 'settings'}
	<SettingsView
		plugins={pluginList}
		onBack={handleBackToPluginList}
		onSavePreferences={handleSavePreferences}
		onGetPreferences={handleGetPreferences}
		{currentPreferences}
	/>
{:else if viewState === 'plugin-running' && rootNode}
	<MainLayout {primaryAction} {secondaryAction} onDispatch={handleDispatch}>
		{#snippet header()}
			<Header
				{rootNode}
				bind:searchText
				onPopView={handlePopView}
				onDispatch={handleDispatch}
				{uiTree}
				showBackButton={true}
			/>
		{/snippet}

		{#snippet content()}
			<Content
				{rootNode}
				{selectedItemNode}
				{uiTree}
				onDispatch={handleDispatch}
				onSelect={handleSelect}
				{searchText}
			/>
		{/snippet}

		{#snippet footer()}
			<Footer
				{uiTree}
				onDispatch={handleDispatch}
				{primaryAction}
				{secondaryAction}
				{actionPanel}
				actions={allActions}
				{navigationTitle}
				{toasts}
				onToastAction={handleToastAction}
				onHideToast={handleHideToast}
			/>
		{/snippet}
	</MainLayout>
{/if}
