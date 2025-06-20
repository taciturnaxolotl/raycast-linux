<script lang="ts">
	import { setContext } from 'svelte';
	import MainLayout from '$lib/components/layout/MainLayout.svelte';
	import Header from '$lib/components/layout/Header.svelte';
	import Content from '$lib/components/layout/Content.svelte';
	import { uiStore } from '$lib/ui.svelte';
	import path from 'path';
	import ActionBar from './nodes/shared/ActionBar.svelte';
	import Footer from './layout/Footer.svelte';
	import NodeRenderer from './NodeRenderer.svelte';

	const {
		uiTree,
		rootNodeId,
		selectedNodeId,
		toasts,
		currentRunningPlugin,
		primaryAction: primaryActionObject,
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
	const toastToShow = $derived(Array.from(toasts.entries()).sort((a, b) => b[0] - a[0])[0]?.[1]);
	const showActionPanelDropdown = $derived((allActions?.length ?? 0) > 1);

	const assetsPath = $derived(
		currentRunningPlugin ? path.dirname(currentRunningPlugin.pluginPath) + '/assets' : ''
	);
	setContext('assetsPath', assetsPath);

	function handleSelect(nodeId: number | undefined) {
		uiStore.selectedNodeId = nodeId;
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			if (event.defaultPrevented) {
				return;
			}
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
	<MainLayout primaryAction={primaryActionObject} {secondaryAction} {onDispatch}>
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
			{#if toastToShow}
				<Footer toast={toastToShow} {onToastAction} />
			{:else}
				<ActionBar title={navigationTitle}>
					{#snippet primaryAction({ props })}
						{#if primaryActionObject}
							<NodeRenderer
								{...props}
								nodeId={primaryActionObject.id}
								{uiTree}
								{onDispatch}
								displayAs="button"
							/>
						{/if}
					{/snippet}
					{#snippet actions()}
						{#if showActionPanelDropdown && actionPanel}
							<NodeRenderer
								nodeId={actionPanel.id}
								{uiTree}
								{onDispatch}
								primaryActionNodeId={primaryActionObject?.id}
								secondaryActionNodeId={secondaryAction?.id}
							/>
						{/if}
					{/snippet}
				</ActionBar>
			{/if}
		{/snippet}
	</MainLayout>
{/if}
