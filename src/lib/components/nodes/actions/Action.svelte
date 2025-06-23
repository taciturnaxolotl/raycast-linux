<script lang="ts">
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import { useActionRole } from '$lib/actions.svelte';
	import BaseAction from './BaseAction.svelte';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: unknown[]) => void;
		displayAs?: 'item' | 'button';
	};

	let { nodeId, uiTree, onDispatch, displayAs = 'item' }: Props = $props();
	const { props: componentProps } = $derived.by(
		useTypedNode(() => ({ nodeId, uiTree, type: 'Action' }))
	);
	const { isPrimaryAction, isSecondaryAction } = $derived.by(useActionRole(nodeId));

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
		{isSecondaryAction}
		{displayAs}
		onclick={handleClick}
	/>
{/if}
