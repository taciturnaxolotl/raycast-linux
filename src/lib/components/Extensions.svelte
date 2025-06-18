<script lang="ts">
	import { StoreListingsReturnTypeSchema, type Datum } from '$lib/store';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { ArrowLeft } from '@lucide/svelte';
	import Icon from './Icon.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import ExtensionListView from './extensions/ExtensionListView.svelte';
	import ExtensionDetailView from './extensions/ExtensionDetailView.svelte';
	import ImageLightbox from './extensions/ImageLightbox.svelte';
	import CategoryFilter from './extensions/CategoryFilter.svelte';

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

	let currentPage = $state(1);
	let perPage = 50;
	let isFetchingMore = $state(false);
	let hasMore = $state(true);

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
			return;
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
			<CategoryFilter {allCategories} bind:selectedCategory />
		{/if}
	</header>

	{#if selectedExtension}
		<ExtensionDetailView
			extension={selectedExtension}
			{isInstalling}
			onInstall={handleInstall}
			onOpenLightbox={(imageUrl) => (expandedImageUrl = imageUrl)}
		/>
	{:else}
		<div class="grow overflow-y-auto" role="listbox" tabindex={0} bind:this={scrollContainer}>
			<ExtensionListView
				{featuredExtensions}
				{trendingExtensions}
				{extensions}
				{searchResults}
				{isLoading}
				{isFetchingMore}
				{error}
				{searchText}
				{selectedCategory}
				bind:selectedIndex
				onSelect={(ext) => (selectedExtension = ext)}
			/>
		</div>
	{/if}
</main>

{#if expandedImageUrl}
	<ImageLightbox imageUrl={expandedImageUrl} onClose={() => (expandedImageUrl = null)} />
{/if}
