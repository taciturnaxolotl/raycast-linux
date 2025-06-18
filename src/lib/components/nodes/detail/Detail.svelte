<script lang="ts">
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import NodeRenderer from '$lib/components/NodeRenderer.svelte';
	import SvelteMarked from 'svelte-marked';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
		layout?: 'horizontal' | 'vertical';
	};

	let { nodeId, uiTree, onDispatch, layout = 'horizontal' }: Props = $props();

	const { node, props: detailProps } = $derived.by(
		useTypedNode(() => ({ nodeId, uiTree, type: ['Detail', 'List.Item.Detail'] }))
	);

	const metadataNodeId = $derived(node?.namedChildren?.['metadata']);
</script>

{#if node && detailProps}
	<div
		class="flex h-full"
		class:flex-row={layout === 'horizontal'}
		class:flex-col={layout === 'vertical'}
	>
		<main
			class="flex-grow overflow-y-auto"
			class:p-6={layout === 'horizontal'}
			class:p-4={layout === 'vertical'}
		>
			{#if detailProps.isLoading}
				<div class="h-2 w-full animate-pulse rounded bg-gray-200"></div>
			{/if}

			{#if detailProps.markdown}
				<article class="prose dark:prose-invert prose-img:mx-auto prose-img:max-w-full max-w-full">
					<SvelteMarked source={detailProps.markdown} />
				</article>
			{/if}
		</main>

		{#if metadataNodeId}
			<aside
				class="shrink-0 overflow-y-auto bg-gray-50/50 p-4 dark:border-gray-800 dark:bg-gray-900/50"
				class:w-72={layout === 'horizontal'}
				class:border-l={layout === 'horizontal'}
				class:border-t={layout === 'vertical'}
			>
				<NodeRenderer nodeId={metadataNodeId} {uiTree} {onDispatch} />
			</aside>
		{/if}
	</div>
{/if}
