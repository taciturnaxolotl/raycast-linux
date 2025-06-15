<script lang="ts">
	import { VList, type VListHandle } from 'virtua/svelte';
	import type { UINode } from '$lib/types';
	import type { ListItemProps } from '$lib/props';
	import ListItem from './ListItem.svelte';
	import ListSection from './ListSection.svelte';
	import { useListView } from '$lib/views';
	import { useTypedNode } from '$lib/node.svelte';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
		onSelect: (nodeId: number | undefined) => void;
		searchText: string;
	};
	let { nodeId, uiTree, onDispatch, onSelect, searchText }: Props = $props();

	const { props: listProps } = $derived.by(useTypedNode(() => ({ nodeId, uiTree, type: 'List' })));

	const view = useListView(() => ({
		nodeId,
		uiTree,
		onSelect,
		searchText,
		filtering: listProps?.filtering,
		onSearchTextChange: !!listProps?.onSearchTextChange
	}));

	const listData = $derived.by(() => {
		const HEADER_HEIGHT = 34;
		const ITEM_HEIGHT = 40;
		return view.flatList.map((item) => ({
			...item,
			height: item.type === 'header' ? HEADER_HEIGHT : ITEM_HEIGHT
		}));
	});

	let vlistInstance: VListHandle | undefined = $state();
	$effect(() => {
		view.vlistInstance = vlistInstance;
	});
</script>

<svelte:window onkeydown={view.handleKeydown} />

<div class="flex h-full flex-col">
	<div class="flex-grow">
		<VList bind:this={vlistInstance} data={listData} getKey={(item) => item.id} class="h-full">
			{#snippet children(item, index)}
				{#if item.type === 'header'}
					<ListSection props={item.props} />
				{:else if item.type === 'item'}
					<ListItem
						props={item.props as ListItemProps}
						selected={view.selectedItemIndex === index}
						onclick={() => view.setSelectedItemIndex(index)}
					/>
				{/if}
			{/snippet}
		</VList>
	</div>
</div>
