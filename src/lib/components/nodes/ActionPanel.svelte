<script lang="ts">
	import type { UINode } from '$lib/types';
	import NodeRenderer from '../NodeRenderer.svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { useTypedNode } from '$lib/node.svelte';
	import { setContext } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { MoreHorizontal } from '@lucide/svelte';
	import { Kbd } from '../ui/kbd';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
		primaryActionNodeId?: number;
		secondaryActionNodeId?: number;
	};

	let { nodeId, uiTree, onDispatch, primaryActionNodeId, secondaryActionNodeId }: Props = $props();
	const { node, props: componentProps } = $derived.by(
		useTypedNode(() => ({
			nodeId,
			uiTree,
			type: 'ActionPanel'
		}))
	);

	const context = $state({
		get primaryActionNodeId() {
			return primaryActionNodeId;
		},
		get secondaryActionNodeId() {
			return secondaryActionNodeId;
		}
	});
	setContext('ActionPanelContext', context);

	let open = $state(false);

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'k' && (e.ctrlKey || e.metaKey)) {
			e.preventDefault();
			open = !open;
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if node && componentProps}
	<DropdownMenu.Root bind:open>
		<DropdownMenu.Trigger>
			{#snippet child({ props })}
				<Button {...props} variant="ghost" size="sm">
					Actions <Kbd>⌘ K</Kbd>
				</Button>
			{/snippet}
		</DropdownMenu.Trigger>
		<DropdownMenu.Content>
			{#each node.children as childId}
				<NodeRenderer nodeId={childId} {uiTree} {onDispatch} />
			{/each}
		</DropdownMenu.Content>
	</DropdownMenu.Root>
{/if}
