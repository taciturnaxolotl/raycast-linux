<script lang="ts">
	import type { UINode } from '$lib/types';
	import type { GridItemProps } from '$lib/props';
	import GridSection from './GridSection.svelte';
	import GridItem from './GridItem.svelte';
	import { useGridView } from '$lib/views';
	import { useTypedNode } from '$lib/node.svelte';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onSelect: (nodeId: number | undefined) => void;
		searchText: string;
	};
	let { nodeId, uiTree, onSelect, searchText }: Props = $props();

	const { props: gridProps } = $derived.by(useTypedNode(() => ({ nodeId, uiTree, type: 'Grid' })));

	const view = useGridView(() => ({
		nodeId,
		uiTree,
		onSelect,
		columns: gridProps?.columns ?? 6,
		searchText,
		filtering: gridProps?.filtering,
		onSearchTextChange: !!gridProps?.onSearchTextChange,
		inset: gridProps?.inset
	}));
</script>

<svelte:window onkeydown={view.handleKeydown} />

<div class="flex h-full flex-col">
	<div class="flex-grow overflow-y-auto">
		<div
			class="grid h-full content-start"
			style:grid-template-columns={`repeat(${gridProps?.columns ?? 6}, 1fr)`}
		>
			{#each view.flatList as item, index (item.id)}
				{#if item.type === 'header'}
					<GridSection props={item.props} />
				{:else if item.type === 'item'}
					<div id="item-{item.id}">
						<GridItem
							props={item.props as GridItemProps}
							selected={view.selectedItemIndex === index}
							onclick={() => view.setSelectedItemIndex(index)}
							inset={item.inset}
						/>
					</div>
				{/if}
			{/each}
		</div>
	</div>
</div>
