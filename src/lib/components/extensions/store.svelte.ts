import { StoreListingsReturnTypeSchema, type Datum } from '$lib/store';

function createExtensionsStore() {
	let extensions = $state<Datum[]>([]);
	let searchResults = $state<Datum[]>([]);
	let featuredExtensions = $state<Datum[]>([]);
	let trendingExtensions = $state<Datum[]>([]);

	let isLoading = $state(true);
	let error = $state<string | null>(null);

	let searchText = $state('');
	let selectedCategory = $state('All Categories');
	let selectedIndex = $state(0);
	let allCategories = $state<string[]>(['All Categories']);

	let currentPage = $state(1);
	const perPage = 50;
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
			} catch (e: unknown) {
				error = e instanceof Error ? e.message : 'Unknown error';
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
			} catch (e: unknown) {
				error = e instanceof Error ? e.message : 'Unknown error';
				console.error(e);
				searchResults = [];
			} finally {
				isLoading = false;
			}
		}, 300);

		return () => clearTimeout(searchDebounceTimer);
	});

	const loadMore = async () => {
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
	};

	return {
		get extensions() {
			return extensions;
		},
		get searchResults() {
			return searchResults;
		},
		get featuredExtensions() {
			return featuredExtensions;
		},
		get trendingExtensions() {
			return trendingExtensions;
		},
		get isLoading() {
			return isLoading;
		},
		get error() {
			return error;
		},
		get searchText() {
			return searchText;
		},
		set searchText(value: string) {
			searchText = value;
		},
		get selectedCategory() {
			return selectedCategory;
		},
		set selectedCategory(value: string) {
			selectedCategory = value;
		},
		get selectedIndex() {
			return selectedIndex;
		},
		set selectedIndex(value: number) {
			selectedIndex = value;
		},
		get allCategories() {
			return allCategories;
		},
		get isFetchingMore() {
			return isFetchingMore;
		},
		loadMore
	};
}

export const extensionsStore = createExtensionsStore();
