<script lang="ts">
	import { DatumSchema, StoreListingsReturnTypeSchema, type Datum } from '$lib/store';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import {
		ArrowLeft,
		Download,
		User,
		Book,
		Boxes,
		ChevronsUpDown,
		ArrowRight,
		Command as CommandIcon,
		Check,
		ArrowUpRight,
		X
	} from '@lucide/svelte';
	import Icon from './Icon.svelte';
	import { openUrl } from '@tauri-apps/plugin-opener';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Command from '$lib/components/ui/command';
	import * as Popover from '$lib/components/ui/popover';
	import { Kbd } from './ui/kbd';
	import z from 'zod';
	import { Separator } from './ui/separator';
	import * as Carousel from '$lib/components/ui/carousel/index.js';
	import { invoke } from '@tauri-apps/api/core';

	type Props = {
		onBack: () => void;
		onInstall: () => void;
	};

	let { onBack, onInstall }: Props = $props();

	let extensions = $state<Datum[]>([]);
	let featuredExtensions = $state<Datum[]>([]);
	let trendingExtensions = $state<Datum[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedExtension = $state<Datum | null>(null);
	let searchText = $state('');
	let selectedIndex = $state(0);
	let expandedImageUrl = $state<string | null>(null);
	let isInstalling = $state(false);

	let allCategories = $state<string[]>(['All Categories']);
	let selectedCategory = $state('All Categories');
	let categoryPopoverOpen = $state(false);

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
		async function fetchAllData() {
			try {
				isLoading = true;
				const [storeRes, featuredRes, trendingRes] = await Promise.all([
					fetch('https://backend.raycast.com/api/v1/store_listings'),
					fetch('https://backend.raycast.com/api/v1/extensions/featured'),
					fetch('https://backend.raycast.com/api/v1/extensions/trending')
				]);

				if (!storeRes.ok) throw new Error(`Store fetch failed: ${storeRes.status}`);
				const storeParsed = StoreListingsReturnTypeSchema.parse(await storeRes.json());
				extensions = storeParsed.data;

				if (featuredRes.ok) {
					const featuredParsed = StoreListingsReturnTypeSchema.parse(await featuredRes.json());
					featuredExtensions = featuredParsed.data;
				}

				if (trendingRes.ok) {
					const trendingParsed = StoreListingsReturnTypeSchema.parse(await trendingRes.json());
					trendingExtensions = trendingParsed.data;
				}

				const categories = new Set<string>();
				for (const ext of extensions) {
					for (const cat of ext.categories) {
						categories.add(cat);
					}
				}
				allCategories = ['All Categories', ...Array.from(categories).sort()];
			} catch (e: any) {
				error = e.message;
				console.error(e);
			} finally {
				isLoading = false;
			}
		}
		fetchAllData();
	});

	const listToDisplay = $derived.by(() => {
		if (searchText) {
			return extensions.filter(
				(ext) =>
					ext.title.toLowerCase().includes(searchText.toLowerCase()) ||
					ext.description.toLowerCase().includes(searchText.toLowerCase()) ||
					ext.author.name.toLowerCase().includes(searchText.toLowerCase())
			);
		}
		if (selectedCategory !== 'All Categories') {
			return extensions.filter((ext) => ext.categories.includes(selectedCategory));
		}
		return [];
	});

	$effect(() => {
		if (searchText || selectedCategory !== 'All Categories') selectedIndex = 0;
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

		if (expandedImageUrl) return;

		if (!selectedExtension) {
			const currentList =
				listToDisplay.length > 0
					? listToDisplay
					: featuredExtensions.concat(trendingExtensions).concat(extensions);
			if (e.key === 'ArrowDown') {
				e.preventDefault();
				selectedIndex = Math.min(currentList.length - 1, selectedIndex + 1);
			} else if (e.key === 'ArrowUp') {
				e.preventDefault();
				selectedIndex = Math.max(0, selectedIndex - 1);
			} else if (e.key === 'Enter') {
				e.preventDefault();
				if (currentList[selectedIndex]) {
					selectedExtension = currentList[selectedIndex];
				}
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
							{#if selectedExtension.categories.includes('AI Extensions')}
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
		<div class="grow overflow-y-auto" role="listbox" tabindex={0}>
			{#snippet renderListItem(ext: Datum, i: number, isSelected: boolean)}
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

			{#if isLoading}
				<div class="text-muted-foreground flex h-full items-center justify-center">
					Loading extensions...
				</div>
			{:else if error}
				<div class="flex h-full items-center justify-center text-red-500">Error: {error}</div>
			{:else if listToDisplay.length > 0}
				{#each listToDisplay as ext, i (ext.id)}
					{@render renderListItem(ext, i, selectedIndex === i)}
				{/each}
			{:else}
				{#if featuredExtensions.length > 0}
					<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">
						Featured
					</h3>
					{#each featuredExtensions as ext, i (ext.id)}
						{@render renderListItem(ext, i, selectedIndex === i)}
					{/each}
				{/if}
				{#if trendingExtensions.length > 0}
					<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">
						Trending
					</h3>
					{#each trendingExtensions as ext, i (ext.id)}
						{@const listIndex = i + featuredExtensions.length}
						{@render renderListItem(ext, listIndex, selectedIndex === listIndex)}
					{/each}
				{/if}
				<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">
					All Extensions
				</h3>
				{#each extensions as ext, i (ext.id)}
					{@const listIndex = i + featuredExtensions.length + trendingExtensions.length}
					{@render renderListItem(ext, listIndex, selectedIndex === listIndex)}
				{/each}
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
