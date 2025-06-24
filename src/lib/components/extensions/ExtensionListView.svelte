<script lang="ts">
	import type { Datum } from '$lib/store';
	import ExtensionListItem from './ExtensionListItem.svelte';
	import { extensionsStore } from './store.svelte';
	import BaseList from '$lib/components/BaseList.svelte';

	type Props = {
		onSelect: (ext: Datum) => void;
	};

	let { onSelect }: Props = $props();

	type DisplayItem = {
		id: string | number;
		itemType: 'header' | 'item';
		data: Datum | string;
	};

	let currentItems = $state<DisplayItem[]>([]);

	$effect(() => {
		const newItems: DisplayItem[] = [];
		const addedIds = new Set<string>();

		const addItems = (exts: Datum[]) => {
			for (const ext of exts) {
				if (!addedIds.has(ext.id)) {
					newItems.push({ id: ext.id, itemType: 'item', data: ext });
					addedIds.add(ext.id);
				}
			}
		};

		if (extensionsStore.searchText) {
			if (extensionsStore.searchResults.length > 0) {
				newItems.push({ id: 'header-search', itemType: 'header', data: 'Search Results' });
				addItems(extensionsStore.searchResults);
			}
		} else if (extensionsStore.selectedCategory !== 'All Categories') {
			const filtered =
				extensionsStore.extensions.filter(
					(ext) => ext.categories?.includes(extensionsStore.selectedCategory) ?? false
				) ?? [];
			if (filtered.length > 0) {
				newItems.push({
					id: `header-${extensionsStore.selectedCategory}`,
					itemType: 'header',
					data: extensionsStore.selectedCategory
				});
				addItems(filtered);
			}
		} else {
			if (extensionsStore.featuredExtensions.length > 0) {
				newItems.push({ id: 'header-featured', itemType: 'header', data: 'Featured' });
				addItems(extensionsStore.featuredExtensions);
			}
			if (extensionsStore.trendingExtensions.length > 0) {
				newItems.push({ id: 'header-trending', itemType: 'header', data: 'Trending' });
				addItems(extensionsStore.trendingExtensions);
			}
			if (extensionsStore.extensions.length > 0) {
				newItems.push({ id: 'header-all', itemType: 'header', data: 'All Extensions' });
				addItems(extensionsStore.extensions);
			}
		}

		if (!extensionsStore.isSearching) {
			currentItems = newItems;
		}
	});
</script>

{#if extensionsStore.error}
	<div class="flex h-full items-center justify-center text-red-500">
		Error: {extensionsStore.error}
	</div>
{:else if currentItems.length === 0}
	{#if !extensionsStore.isSearching}
		<div class="text-muted-foreground flex h-full items-center justify-center">
			{#if extensionsStore.searchText}
				No results for "{extensionsStore.searchText}"
			{:else if extensionsStore.selectedCategory !== 'All Categories'}
				No extensions found in this category.
			{/if}
		</div>
	{/if}
{:else}
	<BaseList
		items={currentItems}
		onenter={(item) => onSelect(item.data as Datum)}
		bind:selectedIndex={extensionsStore.selectedIndex}
		isItemSelectable={(item) => item.itemType === 'item'}
	>
		{#snippet itemSnippet({ item, isSelected, onclick })}
			{#if item.itemType === 'header'}
				<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">
					{item.data}
				</h3>
			{:else if item.itemType === 'item'}
				<ExtensionListItem ext={item.data as Datum} {isSelected} {onclick} />
			{/if}
		{/snippet}
	</BaseList>
{/if}

{#if !extensionsStore.searchText && extensionsStore.isFetchingMore}
	<div class="text-muted-foreground flex h-10 items-center justify-center">Loading more...</div>
{/if}
