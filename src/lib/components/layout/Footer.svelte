<script lang="ts">
	import type { UINode } from '$lib/types';
	import NodeRenderer from '$lib/components/NodeRenderer.svelte';
	import { Separator } from '$lib/components/ui/separator/index.js';

	type Props = {
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
		primaryAction?: UINode;
		secondaryAction?: UINode;
		actionPanel?: UINode;
		actions?: UINode[];
		navigationTitle?: string;
	};
	let {
		uiTree,
		onDispatch,
		primaryAction,
		secondaryAction,
		actionPanel,
		actions,
		navigationTitle
	}: Props = $props();

	const showActionPanelDropdown = $derived((actions?.length ?? 0) > 1);
</script>

<footer class="bg-card flex h-12 shrink-0 items-center border-t px-4">
	{#if navigationTitle}
		<div class="text-muted-foreground text-sm">{navigationTitle}</div>
	{/if}
	<div class="ml-auto">
		{#if actionPanel}
			<div class="group flex items-center">
				{#if primaryAction}
					<NodeRenderer nodeId={primaryAction?.id} {uiTree} {onDispatch} displayAs="button" />
					{#if showActionPanelDropdown}
						<Separator
							orientation="vertical"
							class="!h-4 !w-0.5 !rounded-full transition-opacity group-hover:opacity-0"
						/>
					{/if}
				{/if}
				{#if showActionPanelDropdown}
					<NodeRenderer
						nodeId={actionPanel?.id}
						{uiTree}
						{onDispatch}
						primaryActionNodeId={primaryAction?.id}
						secondaryActionNodeId={secondaryAction?.id}
					/>
				{/if}
			</div>
		{/if}
	</div>
</footer>
