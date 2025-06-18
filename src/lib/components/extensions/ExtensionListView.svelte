<script lang="ts">
	import type { Datum } from '$lib/store';
	import ExtensionListItem from './ExtensionListItem.svelte';
	import { extensionsStore } from './store.svelte';

	type Props = {
		onSelect: (ext: Datum) => void;
	};

	let { onSelect }: Props = $props();

	function handleFocus(index: number) {
		extensionsStore.selectedIndex = index;
	}
</script>

{#if extensionsStore.isLoading && (extensionsStore.extensions.length === 0 || extensionsStore.searchText)}
	<div class="text-muted-foreground flex h-full items-center justify-center">
		Loading extensions...
	</div>
{:else if extensionsStore.error}
	<div class="flex h-full items-center justify-center text-red-500">
		Error: {extensionsStore.error}
	</div>
{:else if extensionsStore.searchText}
	{#if extensionsStore.searchResults.length > 0}
		<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">
			Search Results
		</h3>
		{#each extensionsStore.searchResults as ext, i (ext.id)}
			<button onclick={() => onSelect(ext)} onfocus={() => handleFocus(i)}>
				<ExtensionListItem {ext} isSelected={extensionsStore.selectedIndex === i} />
			</button>
		{/each}
	{:else}
		<div class="text-muted-foreground flex h-full items-center justify-center">
			No results for "{extensionsStore.searchText}"
		</div>
	{/if}
{:else if extensionsStore.selectedCategory !== 'All Categories'}
	{@const filtered = extensionsStore.extensions.filter((ext) =>
		ext.categories?.includes(extensionsStore.selectedCategory)
	)}
	<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">
		{extensionsStore.selectedCategory}
	</h3>
	{#each filtered as ext, i (ext.id)}
		<button onclick={() => onSelect(ext)} onfocus={() => handleFocus(i)}>
			<ExtensionListItem {ext} isSelected={extensionsStore.selectedIndex === i} />
		</button>
	{/each}
	{#if filtered.length === 0}
		<div class="text-muted-foreground flex items-center justify-center p-4">
			No extensions found in this category.
		</div>
	{/if}
{:else}
	{#if extensionsStore.featuredExtensions.length > 0}
		<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">Featured</h3>
		{#each extensionsStore.featuredExtensions as ext, i (ext.id)}
			<button onclick={() => onSelect(ext)} onfocus={() => handleFocus(i)}>
				<ExtensionListItem {ext} isSelected={extensionsStore.selectedIndex === i} />
			</button>
		{/each}
	{/if}
	{#if extensionsStore.trendingExtensions.length > 0}
		<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">Trending</h3>
		{#each extensionsStore.trendingExtensions as ext, i (ext.id)}
			{@const listIndex = i + extensionsStore.featuredExtensions.length}
			<button onclick={() => onSelect(ext)} onfocus={() => handleFocus(listIndex)}>
				<ExtensionListItem {ext} isSelected={extensionsStore.selectedIndex === listIndex} />
			</button>
		{/each}
	{/if}
	<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">
		All Extensions
	</h3>
	{#each extensionsStore.extensions as ext, i (ext.id)}
		{@const listIndex =
			i + extensionsStore.featuredExtensions.length + extensionsStore.trendingExtensions.length}
		<button onclick={() => onSelect(ext)} onfocus={() => handleFocus(listIndex)}>
			<ExtensionListItem {ext} isSelected={extensionsStore.selectedIndex === listIndex} />
		</button>
	{/each}
	{#if extensionsStore.isFetchingMore}
		<div class="text-muted-foreground flex h-10 items-center justify-center">Loading more...</div>
	{/if}
{/if}
