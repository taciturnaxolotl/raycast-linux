<script lang="ts">
	import { StoreListingsReturnTypeSchema, type Datum } from '$lib/store';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { ArrowLeft, Download, ChevronsUpDown, Check, ArrowUpRight, X } from '@lucide/svelte';
	import Icon from './Icon.svelte';
	import { openUrl } from '@tauri-apps/plugin-opener';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Command from '$lib/components/ui/command';
	import * as Popover from '$lib/components/ui/popover';
	import { Kbd } from './ui/kbd';
	import { Separator } from './ui/separator';
	import * as Carousel from '$lib/components/ui/carousel/index.js';
	import { invoke } from '@tauri-apps/api/core';

	type Props = {
		onBack: () => void;
		onInstall: () => void;
	};

	let { onBack, onInstall }: Props = $props();

	let extensions = $state<Datum[]>([]);
	let searchResults = $state<Datum[]>([]);
	let featuredExtensions = $state<Datum[]>([]);
	let trendingExtensions = $state<Datum[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedExtension = $state<Datum | null>(null);
	let searchText = $state('');
	let selectedIndex = $state(0);
	let expandedImageUrl = $state<string | null>(null);
	let isInstalling = $state(false);
	let scrollContainer = $state<HTMLElement | null>(null);

	let allCategories = $state<string[]>(['All Categories']);
	let selectedCategory = $state('All Categories');
	let categoryPopoverOpen = $state(false);

	let currentPage = $state(1);
	let perPage = 50;
	let isFetchingMore = $state(false);
	let hasMore = $state(true);

	function formatTimeAgo(timestamp: number) {
		const date = new Date(timestamp * 1000);
		const now = new Date();
		const seconds = Math.floor((now.getTime() - date.getTime()) / 1000);
		let interval = seconds / 31536000;
		if (interval > 1) {
			const years = Math.floor(interval);
			return `${years} year${years > 1 ? 's' : ''} ago`;
		}
		interval = seconds / 2592000;
		if (interval > 1) {
			const months = Math.floor(interval);
			return `${months} month${months > 1 ? 's' : ''} ago`;
		}
		interval = seconds / 604800;
		if (interval > 1) {
			const weeks = Math.floor(interval);
			return `${weeks} week${weeks > 1 ? 's' : ''} ago`;
		}
		interval = seconds / 86400;
		if (interval > 1) {
			const days = Math.floor(interval);
			return `${days} day${days > 1 ? 's' : ''} ago`;
		}
		interval = seconds / 3600;
		if (interval > 1) {
			const hours = Math.floor(interval);
			return `${hours} hour${hours > 1 ? 's' : ''} ago`;
		}
		interval = seconds / 60;
		if (interval > 1) {
			const minutes = Math.floor(interval);
			return `${minutes} minute${minutes > 1 ? 's' : ''} ago`;
		}
		return `${Math.floor(seconds)} second${seconds !== 1 ? 's' : ''} ago`;
	}

	$effect(() => {
		async function fetchInitialData() {
			try {
				isLoading = true;
				error = null;
				const [storeRes, featuredRes, trendingRes] = await Promise.all([
					fetch(`https://backend.raycast.com/api/v1/store_listings?page=1&per_page=${perPage}`),
					fetch('https://backend.raycast.com/api/v1/extensions/featured'),
					fetch('https://backend.raycast.com/api/v1/extensions/trending')
				]);

				if (!storeRes.ok) throw new Error(`Store fetch failed: ${storeRes.status}`);
				const storeParsed = StoreListingsReturnTypeSchema.parse(await storeRes.json());
				extensions = storeParsed.data;
				currentPage = 1;
				hasMore = storeParsed.data.length === perPage;

				if (featuredRes.ok) {
					const featuredParsed = StoreListingsReturnTypeSchema.parse(await featuredRes.json());
					featuredExtensions = featuredParsed.data;
				} else {
					console.warn(`Featured extensions fetch failed: ${featuredRes.status}`);
				}

				if (trendingRes.ok) {
					const trendingParsed = StoreListingsReturnTypeSchema.parse(await trendingRes.json());
					trendingExtensions = trendingParsed.data;
				} else {
					console.warn(`Trending extensions fetch failed: ${trendingRes.status}`);
				}
			} catch (e: any) {
				error = e.message;
				console.error(e);
			} finally {
				isLoading = false;
			}
		}
		fetchInitialData();
	});

	$effect(() => {
		const categories = new Set<string>();
		const allFetched = [...featuredExtensions, ...trendingExtensions, ...extensions];
		for (const ext of allFetched) {
			if (ext.categories) {
				for (const cat of ext.categories) {
					categories.add(cat);
				}
			}
		}
		allCategories = ['All Categories', ...Array.from(categories).sort()];
	});

	let searchDebounceTimer: NodeJS.Timeout;
	$effect(() => {
		clearTimeout(searchDebounceTimer);
		if (!searchText) {
			searchResults = [];
			if (error) error = null;
			return;
		}

		searchDebounceTimer = setTimeout(async () => {
			isLoading = true;
			error = null;
			try {
				const res = await fetch(
					`https://backend.raycast.com/api/v1/store_listings/search?q=${encodeURIComponent(searchText)}&per_page=${perPage}`
				);
				if (!res.ok) throw new Error(`Search failed: ${res.status}`);
				const parsed = StoreListingsReturnTypeSchema.parse(await res.json());
				searchResults = parsed.data;
				selectedIndex = 0;
			} catch (e: any) {
				error = e.message;
				console.error(e);
				searchResults = [];
			} finally {
				isLoading = false;
			}
		}, 300);

		return () => clearTimeout(searchDebounceTimer);
	});

	async function loadMore() {
		if (isFetchingMore || !hasMore || searchText || selectedCategory !== 'All Categories') return;
		isFetchingMore = true;
		const nextPage = currentPage + 1;
		try {
			const res = await fetch(
				`https://backend.raycast.com/api/v1/store_listings?page=${nextPage}&per_page=${perPage}`
			);
			if (!res.ok) throw new Error('Failed to fetch more extensions');
			const parsed = StoreListingsReturnTypeSchema.parse(await res.json());

			if (parsed.data.length < perPage) {
				hasMore = false;
			}
			const allExtensions = [...extensions, ...parsed.data];
			extensions = [...new Map(allExtensions.map((item) => [item.id, item])).values()];
			currentPage = nextPage;
		} catch (e) {
			console.error('Error loading more extensions:', e);
			hasMore = false;
		} finally {
			isFetchingMore = false;
		}
	}

	$effect(() => {
		const container = scrollContainer;
		if (!container) return;
		const handleScroll = () => {
			if (container.scrollHeight - container.scrollTop - container.clientHeight < 500) {
				loadMore();
			}
		};
		container.addEventListener('scroll', handleScroll);
		return () => container.removeEventListener('scroll', handleScroll);
	});

	function handleGlobalKeyDown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			e.preventDefault();
			if (expandedImageUrl) {
				expandedImageUrl = null;
			} else if (selectedExtension) {
				selectedExtension = null;
			} else {
				onBack();
			}
		}

		if (expandedImageUrl || selectedExtension) return;

		let currentList: Datum[] = [];
		if (searchText) {
			currentList = searchResults;
		} else if (selectedCategory !== 'All Categories') {
			currentList = extensions.filter((ext) => ext.categories.includes(selectedCategory));
		} else {
			currentList = [...featuredExtensions, ...trendingExtensions, ...extensions];
		}

		if (currentList.length === 0) return;

		if (e.key === 'ArrowDown') {
			e.preventDefault();
			selectedIndex = (selectedIndex + 1) % currentList.length;
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			selectedIndex = (selectedIndex - 1 + currentList.length) % currentList.length;
		} else if (e.key === 'Enter') {
			e.preventDefault();
			if (currentList[selectedIndex]) {
				selectedExtension = currentList[selectedIndex];
			}
		}
	}

	async function handleInstall() {
		if (!selectedExtension || isInstalling) return;
		isInstalling = true;
		try {
			await invoke('install_extension', {
				downloadUrl: selectedExtension.download_url,
				slug: selectedExtension.name
			});
			onInstall();
		} catch (e) {
			console.error('Installation failed', e);
		} finally {
			isInstalling = false;
		}
	}
</script>

<svelte:window onkeydown={handleGlobalKeyDown} />

<main class="bg-background text-foreground flex h-screen flex-col">
	<header class="flex h-12 shrink-0 items-center border-b px-2">
		<Button
			variant="ghost"
			size="icon"
			onclick={() => (selectedExtension ? (selectedExtension = null) : onBack())}
		>
			<ArrowLeft class="size-5" />
		</Button>
		{#if selectedExtension}
			<div class="flex items-center gap-3 px-2">
				<Icon
					icon={selectedExtension.icons.light
						? { source: selectedExtension.icons.light, mask: 'Circle' }
						: undefined}
					class="size-6"
				/>
				<h1 class="text-lg font-medium">{selectedExtension.title}</h1>
			</div>
		{:else}
			<Input
				class="rounded-none border-none !bg-transparent pr-0"
				placeholder="Search Store for extensions..."
				bind:value={searchText}
				autofocus
			/>
			<Popover.Root bind:open={categoryPopoverOpen}>
				<Popover.Trigger>
					{#snippet child({ props })}
						<Button
							variant="ghost"
							role="combobox"
							aria-expanded={categoryPopoverOpen}
							class="w-48 justify-between"
							{...props}
						>
							{selectedCategory}
							<ChevronsUpDown class="ml-2 size-4 shrink-0 opacity-50" />
						</Button>
					{/snippet}
				</Popover.Trigger>
				<Popover.Content class="w-48 p-0">
					<Command.Root>
						<Command.Input placeholder="Search category..." />
						<Command.Empty>No category found.</Command.Empty>
						<Command.List>
							{#each allCategories as category}
								<Command.Item
									value={category}
									onSelect={() => {
										selectedCategory = category;
										categoryPopoverOpen = false;
										selectedIndex = 0;
									}}
								>
									<Check class={selectedCategory !== category ? 'text-transparent' : ''} />
									{category}
								</Command.Item>
							{/each}
						</Command.List>
					</Command.Root>
				</Popover.Content>
			</Popover.Root>
		{/if}
	</header>

	{#if selectedExtension}
		<div class="grow overflow-y-auto">
			<div class="mx-auto max-w-4xl p-6">
				<div class="flex items-center gap-4">
					<Icon
						icon={selectedExtension.icons.light
							? { source: selectedExtension.icons.light, mask: 'Circle' }
							: undefined}
						class="size-16"
					/>
					<div>
						<h1 class="text-3xl font-bold">{selectedExtension.title}</h1>
						<div class="mt-2 flex items-center gap-4 text-gray-400">
							<div class="flex items-center gap-1.5">
								<Icon
									icon={selectedExtension.author.avatar
										? { source: selectedExtension.author.avatar, mask: 'Circle' }
										: undefined}
									class="size-5"
								/>
								<span>{selectedExtension.author.name}</span>
							</div>
							<div class="flex items-center gap-1.5">
								<Download class="size-4" />
								<span>{selectedExtension.download_count.toLocaleString()} Installs</span>
							</div>
							{#if selectedExtension.categories?.includes('AI Extensions')}
								<div
									class="flex items-center gap-1.5 rounded bg-purple-500/20 px-1.5 py-0.5 text-xs text-purple-300"
								>
									<Icon icon="command-symbol-16" class="size-3" />
									<span>AI Extension</span>
								</div>
							{/if}
						</div>
					</div>
				</div>

				{#if selectedExtension.metadata_count > 0}
					<Carousel.Root class="mt-8 w-full">
						<Carousel.Content>
							{#each Array(selectedExtension.metadata_count) as _, i}
								<Carousel.Item class="basis-1/3">
									{@const imageUrl = `${selectedExtension.readme_assets_path}metadata/${selectedExtension.name}-${i + 1}.png`}
									<button
										class="w-full cursor-pointer"
										onclick={() => (expandedImageUrl = imageUrl)}
									>
										<img
											src={imageUrl}
											alt={`Screenshot ${i + 1} for ${selectedExtension.title}`}
											class="aspect-video w-full rounded-lg bg-white/5 object-cover"
											loading="lazy"
										/>
									</button>
								</Carousel.Item>
							{/each}
						</Carousel.Content>
						<Carousel.Previous />
						<Carousel.Next />
					</Carousel.Root>
				{/if}

				<div class="mt-8 grid grid-cols-[1fr_auto_1fr] gap-x-8">
					<div class="flex flex-col gap-4">
						<div>
							<h2 class="text-muted-foreground mb-1 text-xs font-medium uppercase">Description</h2>
							<p>{selectedExtension.description}</p>
						</div>

						<Separator />

						<div>
							<h2 class="text-muted-foreground mb-2 text-xs font-medium uppercase">Commands</h2>
							<div class="flex flex-col gap-4">
								{#each selectedExtension.commands as command (command.id)}
									<div class="flex items-start gap-3">
										<Icon
											icon={command.icons.light
												? { source: command.icons.light, mask: 'Circle' }
												: undefined}
											class="mt-1 size-5"
										/>
										<div>
											<p class="mb-1 text-sm font-medium">{command.title}</p>
											<p class="text-muted-foreground text-xs">{command.description}</p>
										</div>
									</div>
								{/each}
							</div>
						</div>
					</div>
					<Separator orientation="vertical" />
					<div class="space-y-8">
						<div>
							<h2 class="text-muted-foreground mb-1 text-xs font-medium uppercase">README</h2>
							<Button
								variant="ghost"
								class="-mx-3 w-full justify-between"
								onclick={() =>
									selectedExtension?.readme_url && openUrl(selectedExtension.readme_url)}
							>
								Open README <ArrowUpRight class="text-muted-foreground size-4" />
							</Button>
						</div>
						<div>
							<h3 class="text-muted-foreground mb-1 text-xs font-medium uppercase">Last updated</h3>
							<p>{formatTimeAgo(selectedExtension.updated_at)}</p>
						</div>
						<div>
							<h3 class="text-muted-foreground mb-1 text-xs font-medium uppercase">Contributors</h3>
							<div class="flex flex-wrap gap-2">
								{#each selectedExtension.contributors as contributor (contributor.handle)}
									<a
										href={`https://github.com/${contributor.github_handle}`}
										target="_blank"
										class="flex items-center gap-2"
										rel="noopener noreferrer"
									>
										<Icon
											icon={contributor.avatar
												? { source: contributor.avatar, mask: 'Circle' }
												: undefined}
											class="size-6"
										/>
									</a>
								{/each}
							</div>
						</div>
						{#if selectedExtension.categories?.length > 0}
							<div>
								<h3 class="text-muted-foreground mb-1 text-xs font-medium uppercase">Categories</h3>
								<div class="flex flex-wrap gap-1.5">
									{#each selectedExtension.categories as category}
										<span
											class="rounded-full bg-blue-900/50 px-2 py-0.5 text-xs font-semibold text-blue-300"
										>
											{category}
										</span>
									{/each}
								</div>
							</div>
						{/if}
						<div>
							<h3 class="text-muted-foreground mb-1 text-xs font-medium uppercase">Source Code</h3>
							<Button
								variant="ghost"
								class="-mx-3 w-full justify-between"
								onclick={() =>
									selectedExtension?.source_url && openUrl(selectedExtension.source_url)}
							>
								View Code <ArrowUpRight class="text-muted-foreground size-4" />
							</Button>
						</div>
					</div>
				</div>
			</div>
		</div>
		<footer class="bg-card flex h-12 shrink-0 items-center justify-between border-t px-4">
			<div class="flex items-center gap-2">
				<Icon
					icon={selectedExtension.icons.light
						? { source: selectedExtension.icons.light, mask: 'Circle' }
						: undefined}
					class="size-5"
				/>
				<span class="text-sm">{selectedExtension.title}</span>
			</div>
			<div class="flex items-center gap-2">
				<Button size="sm" onclick={handleInstall} disabled={isInstalling}>
					{isInstalling ? 'Installing...' : 'Install Extension'}
				</Button>
				<Button variant="outline" size="sm">Actions <Kbd>⌘ K</Kbd></Button>
			</div>
		</footer>
	{:else}
		<div class="grow overflow-y-auto" role="listbox" tabindex={0} bind:this={scrollContainer}>
			{#snippet renderListItem(ext: Datum, i: number, listName: string)}
				{@const isSelected = selectedIndex === i}
				<button
					type="button"
					class="hover:bg-accent/50 flex w-full items-center gap-3 px-4 py-2 text-left"
					class:bg-accent={isSelected}
					onclick={() => (selectedExtension = ext)}
					onfocus={() => (selectedIndex = i)}
				>
					<Icon
						icon={ext.icons.light ? { source: ext.icons.light, mask: 'Circle' } : undefined}
						class="size-6"
					/>
					<div class="flex-grow overflow-hidden">
						<p class="font-medium">{ext.title}</p>
						<p class="text-muted-foreground truncate text-sm">{ext.description}</p>
					</div>
					<div class="ml-auto flex shrink-0 items-center gap-4">
						{#if ext.commands.length > 0}
							<span class="text-muted-foreground text-sm">{ext.commands.length}</span>
						{/if}
						<div class="text-muted-foreground flex items-center gap-1 text-sm">
							<Download class="size-4" />
							{ext.download_count.toLocaleString()}
						</div>
						<Icon
							icon={ext.author.avatar ? { source: ext.author.avatar, mask: 'Circle' } : undefined}
							class="size-6"
						/>
					</div>
				</button>
			{/snippet}

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
						{@render renderListItem(ext, i, 'search')}
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
					{@render renderListItem(ext, i, 'category')}
				{/each}
				{#if filtered.length === 0}
					<div class="text-muted-foreground flex items-center justify-center p-4">
						No extensions found in this category.
					</div>
				{/if}
			{:else}
				{#if featuredExtensions.length > 0}
					<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">
						Featured
					</h3>
					{#each featuredExtensions as ext, i (ext.id)}
						{@render renderListItem(ext, i, 'featured')}
					{/each}
				{/if}
				{#if trendingExtensions.length > 0}
					<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">
						Trending
					</h3>
					{#each trendingExtensions as ext, i (ext.id)}
						{@const listIndex = i + featuredExtensions.length}
						{@render renderListItem(ext, listIndex, 'trending')}
					{/each}
				{/if}
				<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">
					All Extensions
				</h3>
				{#each extensions as ext, i (ext.id)}
					{@const listIndex = i + featuredExtensions.length + trendingExtensions.length}
					{@render renderListItem(ext, listIndex, 'all')}
				{/each}
				{#if isFetchingMore}
					<div class="text-muted-foreground flex h-10 items-center justify-center">
						Loading more...
					</div>
				{/if}
			{/if}
		</div>
	{/if}
</main>

{#if expandedImageUrl}
	<div
		role="dialog"
		aria-modal="true"
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/80"
		onclick={() => (expandedImageUrl = null)}
		tabindex={0}
	>
		<img
			src={expandedImageUrl}
			alt="Expanded screenshot"
			class="max-h-[80vh] max-w-[80vw] object-contain"
		/>
		<Button
			variant="ghost"
			size="icon"
			class="absolute top-4 right-4 text-white hover:text-white"
			onclick={() => (expandedImageUrl = null)}
		>
			<X class="size-6" />
		</Button>
	</div>
{/if}
