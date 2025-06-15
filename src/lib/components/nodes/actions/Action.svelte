<script lang="ts">
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import { getContext } from 'svelte';
	import BaseAction from './BaseAction.svelte';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
		displayAs?: 'item' | 'button';
	};

	let { nodeId, uiTree, onDispatch, displayAs = 'item' }: Props = $props();
	const { props: componentProps } = $derived.by(
		useTypedNode(() => ({ nodeId, uiTree, type: 'Action' }))
	);

	const context: { primaryActionNodeId?: number } | undefined = getContext('ActionPanelContext');
	const isPrimaryAction = $derived(context?.primaryActionNodeId === nodeId);

	function handleClick() {
		onDispatch(nodeId, 'onAction', []);
	}
</script>

{#if componentProps}
	<BaseAction
		title={componentProps.title}
		icon={componentProps.icon}
		shortcut={componentProps.shortcut}
		{isPrimaryAction}
		{displayAs}
		onclick={handleClick}
	/>
{/if}
