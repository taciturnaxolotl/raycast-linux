import { StoreListingsReturnTypeSchema, type Datum } from '$lib/store';

export class ExtensionsStore {
	extensions = $state<Datum[]>([]);
	searchResults = $state<Datum[]>([]);
	featuredExtensions = $state<Datum[]>([]);
	trendingExtensions = $state<Datum[]>([]);

	isLoading = $state(true);
	error = $state<string | null>(null);

	#_searchText = $state('');
	selectedCategory = $state('All Categories');
	selectedIndex = $state(0);

	currentPage = $state(1);
	isFetchingMore = $state(false);
	hasMore = $state(true);

	readonly perPage = 50;
	#searchDebounceTimer: ReturnType<typeof setTimeout> | undefined;

	allCategories = $derived.by(() => {
		const categories = new Set<string>();
		const allFetched = [...this.featuredExtensions, ...this.trendingExtensions, ...this.extensions];
		for (const ext of allFetched) {
			if (ext.categories) {
				for (const cat of ext.categories) {
					categories.add(cat);
				}
			}
		}
		return ['All Categories', ...Array.from(categories).sort()];
	});

	constructor() {
		this.#fetchInitialData();
	}

	get searchText() {
		return this.#_searchText;
	}

	set searchText(value: string) {
		this.#_searchText = value;
		clearTimeout(this.#searchDebounceTimer);

		if (!value) {
			this.searchResults = [];
			if (this.error) this.error = null;
			return;
		}

		this.#searchDebounceTimer = setTimeout(async () => {
			this.isLoading = true;
			this.error = null;
			try {
				const res = await fetch(
					`https://backend.raycast.com/api/v1/store_listings/search?q=${encodeURIComponent(value)}&per_page=${this.perPage}`
				);
				if (!res.ok) throw new Error(`Search failed: ${res.status}`);
				const parsed = StoreListingsReturnTypeSchema.parse(await res.json());
				this.searchResults = parsed.data;
				this.selectedIndex = 0;
			} catch (e: unknown) {
				this.error = e instanceof Error ? e.message : 'Unknown error';
				console.error(e);
				this.searchResults = [];
			} finally {
				this.isLoading = false;
			}
		}, 300);
	}

	async #fetchInitialData() {
		try {
			this.isLoading = true;
			this.error = null;
			const [storeRes, featuredRes, trendingRes] = await Promise.all([
				fetch(`https://backend.raycast.com/api/v1/store_listings?page=1&per_page=${this.perPage}`),
				fetch('https://backend.raycast.com/api/v1/extensions/featured'),
				fetch('https://backend.raycast.com/api/v1/extensions/trending')
			]);

			if (!storeRes.ok) throw new Error(`Store fetch failed: ${storeRes.status}`);
			const storeParsed = StoreListingsReturnTypeSchema.parse(await storeRes.json());
			this.extensions = storeParsed.data;
			this.currentPage = 1;
			this.hasMore = storeParsed.data.length === this.perPage;

			if (featuredRes.ok) {
				const featuredParsed = StoreListingsReturnTypeSchema.parse(await featuredRes.json());
				this.featuredExtensions = featuredParsed.data;
			}

			if (trendingRes.ok) {
				const trendingParsed = StoreListingsReturnTypeSchema.parse(await trendingRes.json());
				this.trendingExtensions = trendingParsed.data;
			}
		} catch (e: unknown) {
			this.error = e instanceof Error ? e.message : 'Unknown error';
			console.error(e);
		} finally {
			this.isLoading = false;
		}
	}

	loadMore = async () => {
		if (
			this.isFetchingMore ||
			!this.hasMore ||
			this.searchText ||
			this.selectedCategory !== 'All Categories'
		) {
			return;
		}

		this.isFetchingMore = true;
		const nextPage = this.currentPage + 1;

		try {
			const res = await fetch(
				`https://backend.raycast.com/api/v1/store_listings?page=${nextPage}&per_page=${this.perPage}`
			);
			if (!res.ok) throw new Error('Failed to fetch more extensions');
			const parsed = StoreListingsReturnTypeSchema.parse(await res.json());

			if (parsed.data.length < this.perPage) {
				this.hasMore = false;
			}

			const allExtensions = [...this.extensions, ...parsed.data];
			this.extensions = [...new Map(allExtensions.map((item) => [item.id, item])).values()];
			this.currentPage = nextPage;
		} catch (e) {
			console.error('Error loading more extensions:', e);
			this.hasMore = false;
		} finally {
			this.isFetchingMore = false;
		}
	};
}

export const extensionsStore = new ExtensionsStore();
