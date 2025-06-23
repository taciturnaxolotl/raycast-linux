<script lang="ts">
	import type { UINode } from '$lib/types';
	import NodeRenderer from '../NodeRenderer.svelte';
	import { useTypedNode } from '$lib/node.svelte';
	import ActionMenu from './shared/ActionMenu.svelte';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: unknown[]) => void;
		primaryActionNodeId?: number;
		secondaryActionNodeId?: number;
	};

	let { nodeId, uiTree, onDispatch, primaryActionNodeId, secondaryActionNodeId }: Props = $props();
	const { node } = $derived.by(
		useTypedNode(() => ({
			nodeId,
			uiTree,
			type: 'ActionPanel'
		}))
	);
</script>

{#if node}
	<ActionMenu {primaryActionNodeId} {secondaryActionNodeId}>
		{#each node.children as childId (childId)}
			<NodeRenderer nodeId={childId} {uiTree} {onDispatch} />
		{/each}
	</ActionMenu>
{/if}
