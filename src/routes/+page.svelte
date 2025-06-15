<script lang="ts">
	import { sidecarService } from '$lib/sidecar.svelte';
	import { uiStore } from '$lib/ui.svelte';
	import type { UINode } from '$lib/types';
	import { untrack } from 'svelte';
	import MainLayout from '$lib/components/layout/MainLayout.svelte';
	import Header from '$lib/components/layout/Header.svelte';
	import Content from '$lib/components/layout/Content.svelte';
	import Footer from '$lib/components/layout/Footer.svelte';
	import PluginList from '$lib/components/PluginList.svelte';
	import type { PluginInfo } from '@raycast-linux/protocol';

	type ViewState = 'plugin-list' | 'plugin-running';

	let viewState = $state<ViewState>('plugin-list');

	const { uiTree, rootNodeId, selectedNodeId, pluginList } = $derived(uiStore);

	$effect(() => {
		untrack(() => {
			sidecarService.setOnGoBackToPluginList(() => {
				viewState = 'plugin-list';
			});
			sidecarService.start();
			return () => sidecarService.stop();
		});
	});

	const rootNode = $derived(uiTree.get(rootNodeId!));
	const selectedItemNode = $derived(uiTree.get(selectedNodeId!));
	let searchText = $state('');
	const navigationTitle = $derived(rootNode?.props.navigationTitle as string | undefined);

	const actionInfo = $derived.by(() => {
		const actionsNodeId =
			rootNode?.type === 'Detail'
				? rootNode.namedChildren?.['actions']
				: selectedItemNode?.namedChildren?.['actions'];
		if (!actionsNodeId)
			return { primary: undefined, secondary: undefined, panel: undefined, allActions: [] };
		const panelNode = uiTree.get(actionsNodeId);
		if (!panelNode || panelNode.type !== 'ActionPanel')
			return { primary: undefined, secondary: undefined, panel: undefined, allActions: [] };

		const foundActions: UINode[] = [];
		function findActions(nodeId: number) {
			const node = uiTree.get(nodeId);
			if (!node) return;
			if (node.type.startsWith('Action.') && !node.type.includes('Panel')) {
				foundActions.push(node);
			} else if (node.type.includes('Panel')) {
				for (const childId of node.children) findActions(childId);
			}
		}
		findActions(actionsNodeId);
		return {
			primary: foundActions[0],
			secondary: foundActions[1],
			panel: panelNode,
			allActions: foundActions
		};
	});

	function handleDispatch(instanceId: number, handlerName: string, args: any[]) {
		sidecarService.dispatchEvent('dispatch-event', { instanceId, handlerName, args });
	}

	function handleSelect(nodeId: number | undefined) {
		uiStore.selectedNodeId = nodeId;
	}

	function handlePopView() {
		sidecarService.dispatchEvent('pop-view');
	}

	function getActionHandlerName(type: string): string {
		switch (type) {
			case 'Action.CopyToClipboard':
				return 'onCopy';
			case 'Action.OpenInBrowser':
				return 'onOpenInBrowser';
			default:
				return 'onAction';
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (viewState !== 'plugin-running') return;

		if (event.key === 'Escape') {
			handlePopView();
			return;
		}
		if (event.key === 'Enter') {
			if (
				event.target instanceof HTMLElement &&
				event.target.closest('[data-slot="dropdown-menu-content"]')
			) {
				return;
			}

			if (event.ctrlKey && !event.metaKey && !event.shiftKey) {
				if (actionInfo.secondary) {
					event.preventDefault();
					const handlerName = getActionHandlerName(actionInfo.secondary.type);
					handleDispatch(actionInfo.secondary.id, handlerName, []);
				}
			} else if (!event.metaKey && !event.ctrlKey && !event.shiftKey) {
				if (actionInfo.primary) {
					event.preventDefault();
					const handlerName = getActionHandlerName(actionInfo.primary.type);
					handleDispatch(actionInfo.primary.id, handlerName, []);
				}
			}
		}
	}

	$effect(() => {
		if (viewState === 'plugin-running' && rootNode) {
			handleDispatch(rootNode.id, 'onSearchTextChange', [searchText]);
		}
	});

	function handleRunPlugin(plugin: PluginInfo) {
		uiStore.resetForNewPlugin();
		sidecarService.dispatchEvent('run-plugin', {
			pluginPath: plugin.pluginPath,
			commandName: plugin.commandName
		});
		viewState = 'plugin-running';
		searchText = '';
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if viewState === 'plugin-list'}
	<PluginList plugins={pluginList} onRunPlugin={handleRunPlugin} />
{:else if viewState === 'plugin-running' && rootNode}
	<MainLayout>
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
				primaryAction={actionInfo.primary}
				secondaryAction={actionInfo.secondary}
				actionPanel={actionInfo.panel}
				actions={actionInfo.allActions}
				{navigationTitle}
			/>
		{/snippet}
	</MainLayout>
{/if}
