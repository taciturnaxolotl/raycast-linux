<script lang="ts">
	import type { Datum } from '$lib/store';
	import ExtensionListItem from './ExtensionListItem.svelte';

	type Props = {
		featuredExtensions: Datum[];
		trendingExtensions: Datum[];
		extensions: Datum[];
		searchResults: Datum[];
		isLoading: boolean;
		isFetchingMore: boolean;
		error: string | null;
		searchText: string;
		selectedCategory: string;
		selectedIndex: number;
		onSelect: (ext: Datum) => void;
	};

	let {
		featuredExtensions,
		trendingExtensions,
		extensions,
		searchResults,
		isLoading,
		isFetchingMore,
		error,
		searchText,
		selectedCategory,
		selectedIndex = $bindable(),
		onSelect
	}: Props = $props();
</script>

{#if isLoading && (extensions.length === 0 || searchText)}
	<div class="text-muted-foreground flex h-full items-center justify-center">
		Loading extensions...
	</div>
{:else if error}
	<div class="flex h-full items-center justify-center text-red-500">Error: {error}</div>
{:else if searchText}
	{#if searchResults.length > 0}
		<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">
			Search Results
		</h3>
		{#each searchResults as ext, i (ext.id)}
			<div onclick={() => onSelect(ext)} onfocus={() => (selectedIndex = i)}>
				<ExtensionListItem {ext} isSelected={selectedIndex === i} />
			</div>
		{/each}
	{:else}
		<div class="text-muted-foreground flex h-full items-center justify-center">
			No results for "{searchText}"
		</div>
	{/if}
{:else if selectedCategory !== 'All Categories'}
	{@const filtered = extensions.filter((ext) => ext.categories?.includes(selectedCategory))}
	<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">
		{selectedCategory}
	</h3>
	{#each filtered as ext, i (ext.id)}
		<div onclick={() => onSelect(ext)} onfocus={() => (selectedIndex = i)}>
			<ExtensionListItem {ext} isSelected={selectedIndex === i} />
		</div>
	{/each}
	{#if filtered.length === 0}
		<div class="text-muted-foreground flex items-center justify-center p-4">
			No extensions found in this category.
		</div>
	{/if}
{:else}
	{#if featuredExtensions.length > 0}
		<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">Featured</h3>
		{#each featuredExtensions as ext, i (ext.id)}
			<div onclick={() => onSelect(ext)} onfocus={() => (selectedIndex = i)}>
				<ExtensionListItem {ext} isSelected={selectedIndex === i} />
			</div>
		{/each}
	{/if}
	{#if trendingExtensions.length > 0}
		<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">Trending</h3>
		{#each trendingExtensions as ext, i (ext.id)}
			{@const listIndex = i + featuredExtensions.length}
			<div onclick={() => onSelect(ext)} onfocus={() => (selectedIndex = listIndex)}>
				<ExtensionListItem {ext} isSelected={selectedIndex === listIndex} />
			</div>
		{/each}
	{/if}
	<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">
		All Extensions
	</h3>
	{#each extensions as ext, i (ext.id)}
		{@const listIndex = i + featuredExtensions.length + trendingExtensions.length}
		<div onclick={() => onSelect(ext)} onfocus={() => (selectedIndex = listIndex)}>
			<ExtensionListItem {ext} isSelected={selectedIndex === listIndex} />
		</div>
	{/each}
	{#if isFetchingMore}
		<div class="text-muted-foreground flex h-10 items-center justify-center">Loading more...</div>
	{/if}
{/if}
