<script lang="ts">
	import { setContext } from 'svelte';
	import MainLayout from '$lib/components/layout/MainLayout.svelte';
	import Header from '$lib/components/layout/Header.svelte';
	import Content from '$lib/components/layout/Content.svelte';
	import Footer from '$lib/components/layout/Footer.svelte';
	import { uiStore } from '$lib/ui.svelte';
	import path from 'path';

	const {
		uiTree,
		rootNodeId,
		selectedNodeId,
		toasts,
		currentRunningPlugin,
		primaryAction,
		secondaryAction,
		actionPanel,
		allActions
	} = $derived(uiStore);

	type Props = {
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
		onPopView: () => void;
		onToastAction: (toastId: number, actionType: 'primary' | 'secondary') => void;
		onHideToast: (toastId: number) => void;
	};

	let { onDispatch, onPopView, onToastAction, onHideToast }: Props = $props();

	const rootNode = $derived(uiTree.get(rootNodeId!));
	const selectedItemNode = $derived(uiTree.get(selectedNodeId!));
	let searchText = $state('');
	const navigationTitle = $derived(rootNode?.props.navigationTitle as string | undefined);

	const assetsPath = $derived(
		currentRunningPlugin ? path.dirname(currentRunningPlugin.pluginPath) + '/assets' : ''
	);
	setContext('assetsPath', assetsPath);

	function handleSelect(nodeId: number | undefined) {
		uiStore.selectedNodeId = nodeId;
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			onPopView();
			return;
		}
	}

	$effect(() => {
		if (rootNode) {
			onDispatch(rootNode.id, 'onSearchTextChange', [searchText]);
		}
	});
</script>

<svelte:window onkeydown={handleKeydown} />

{#if rootNode}
	<MainLayout {primaryAction} {secondaryAction} {onDispatch}>
		{#snippet header()}
			<Header {rootNode} bind:searchText {onPopView} {onDispatch} {uiTree} showBackButton={true} />
		{/snippet}

		{#snippet content()}
			<Content
				{rootNode}
				{selectedItemNode}
				{uiTree}
				{onDispatch}
				onSelect={handleSelect}
				{searchText}
			/>
		{/snippet}

		{#snippet footer()}
			<Footer
				{uiTree}
				{onDispatch}
				{primaryAction}
				{secondaryAction}
				{actionPanel}
				actions={allActions}
				{navigationTitle}
				{toasts}
				{onToastAction}
				{onHideToast}
			/>
		{/snippet}
	</MainLayout>
{/if}
