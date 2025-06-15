<script lang="ts">
	import type { UINode } from '$lib/types';
	import NodeRenderer from '$lib/components/NodeRenderer.svelte';
	import { Separator } from '$lib/components/ui/separator/index.js';

	type Props = {
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
		primaryAction?: UINode;
		actionPanel?: UINode;
	};
	let { uiTree, onDispatch, primaryAction, actionPanel }: Props = $props();
</script>

<footer class="bg-card flex h-12 shrink-0 items-center justify-end border-t px-4">
	{#if actionPanel}
		<div class="group flex items-center">
			{#if primaryAction}
				<NodeRenderer nodeId={primaryAction?.id} {uiTree} {onDispatch} displayAs="button" />
				<Separator
					orientation="vertical"
					class="!h-4 !w-0.5 !rounded-full transition-opacity group-hover:opacity-0"
				/>
			{/if}
			<NodeRenderer
				nodeId={actionPanel?.id}
				{uiTree}
				{onDispatch}
				primaryActionNodeId={primaryAction?.id}
			/>
		</div>
	{/if}
</footer>
