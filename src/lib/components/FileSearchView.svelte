<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { untrack } from 'svelte';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { ArrowLeft, Trash, Loader2, Folder, File, Copy, ArrowUpRight, Eye } from '@lucide/svelte';
	import ListItemBase from './nodes/shared/ListItemBase.svelte';
	import { writeText } from '@tauri-apps/plugin-clipboard-manager';
	import { Kbd } from './ui/kbd';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { shortcutToText } from '$lib/renderKey';
	import ActionBar from './nodes/shared/ActionBar.svelte';
	import ActionMenu from './nodes/shared/ActionMenu.svelte';
	import BaseList from './BaseList.svelte';
	import { open } from '@tauri-apps/plugin-shell';

	type Props = {
		onBack: () => void;
	};

	type IndexedFile = {
		path: string;
		name: string;
		parentPath: string;
		fileType: 'file' | 'directory';
		lastModified: number; // unix timestamp
	};

	let { onBack }: Props = $props();

	let searchResults = $state<IndexedFile[]>([]);
	let selectedIndex = $state(0);
	let searchText = $state('');
	let isFetching = $state(false);

	const selectedItem = $derived(searchResults[selectedIndex]);

	const fetchFiles = async () => {
		if (isFetching) return;
		isFetching = true;
		try {
			const newItems = await invoke<IndexedFile[]>('search_files', {
				term: searchText
			});
			searchResults = newItems;
			if (selectedIndex >= newItems.length) {
				selectedIndex = 0;
			}
		} catch (e) {
			console.error('Failed to fetch files:', e);
		} finally {
			isFetching = false;
		}
	};

	const formatDateTime = (timestamp: number) => {
		const date = new Date(timestamp * 1000);
		if (date.getFullYear() < 1971) return 'N/A';
		return date.toLocaleString();
	};

	const handleOpen = async (item: IndexedFile) => {
		await open(item.path);
		onBack();
	};

	const handleShow = async (item: IndexedFile) => {
		await invoke('show_in_finder', { path: item.path });
	};

	const handleCopyPath = async (item: IndexedFile) => {
		await writeText(item.path);
	};

	const handleDelete = async (item: IndexedFile) => {
		await invoke('trash', { paths: [item.path] });
		fetchFiles();
	};

	const handleKeydown = (e: KeyboardEvent) => {
		if (e.key === 'Escape') {
			e.preventDefault();
			onBack();
			return;
		}
		if (!selectedItem) return;

		if (e.ctrlKey && e.key.toLowerCase() === 'c') {
			e.preventDefault();
			handleCopyPath(selectedItem);
		}

		if (e.ctrlKey && e.key.toLowerCase() === 'x') {
			e.preventDefault();
			handleDelete(selectedItem);
		}

		if (e.metaKey && e.key === 'Enter') {
			e.preventDefault();
			handleShow(selectedItem);
		}
	};

	$effect(() => {
		const term = searchText;
		if (!term) {
			searchResults = [];
			isFetching = false;
			return;
		}

		untrack(() => {
			const timer = setTimeout(() => {
				if (term === searchText) {
					fetchFiles();
				}
			}, 200);
			return () => clearTimeout(timer);
		});
	});
</script>

<svelte:window onkeydown={handleKeydown} />

<main class="bg-background text-foreground flex h-screen flex-col">
	<header class="flex h-12 shrink-0 items-center border-b px-2">
		<Button variant="ghost" size="icon" onclick={onBack}>
			<ArrowLeft class="size-5" />
		</Button>
		<Input
			class="rounded-none border-none !bg-transparent pr-0"
			placeholder="Search for files and folders..."
			bind:value={searchText}
			autofocus
		/>
	</header>
	<div class="grid grow grid-cols-[minmax(0,_1.5fr)_minmax(0,_2.5fr)] overflow-y-hidden">
		<div class="flex-grow overflow-y-auto border-r">
			{#if isFetching && searchResults.length === 0}
				<div class="text-muted-foreground flex h-full items-center justify-center">
					<Loader2 class="size-6 animate-spin" />
				</div>
			{/if}
			<BaseList
				items={searchResults.map((item) => ({ ...item, id: item.path }))}
				bind:selectedIndex
				onenter={(item) => handleOpen(item)}
			>
				{#snippet itemSnippet({ item, isSelected, onclick })}
					<button class="w-full text-left" {onclick}>
						<ListItemBase
							icon={item.fileType === 'directory' ? 'folder-16' : 'blank-document-16'}
							title={item.name}
							subtitle={item.parentPath}
							{isSelected}
						/>
					</button>
				{/snippet}
			</BaseList>
		</div>
		<div class="flex flex-col overflow-y-hidden">
			{#if selectedItem}
				<div class="flex h-full flex-col items-center justify-center p-4">
					<div class="mb-4">
						{#if selectedItem.fileType === 'directory'}
							<Folder class="size-24 text-gray-500" />
						{:else}
							<File class="size-24 text-gray-500" />
						{/if}
					</div>
					<p class="text-xl font-semibold">{selectedItem.name}</p>
					<p class="text-muted-foreground text-sm">{selectedItem.path}</p>
				</div>

				<div class="border-t p-4">
					<h3 class="text-muted-foreground mb-2 text-xs font-semibold uppercase">Information</h3>
					<div class="flex flex-col gap-3 text-sm">
						<div class="flex justify-between">
							<span class="text-muted-foreground">Type</span>
							<span class="capitalize">{selectedItem.fileType}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-muted-foreground">Last Modified</span>
							<span>{formatDateTime(selectedItem.lastModified)}</span>
						</div>
					</div>
				</div>

				<ActionBar>
					{#snippet primaryAction({ props })}
						<Button {...props} onclick={() => handleOpen(selectedItem)}>
							Open <Kbd>⏎</Kbd>
						</Button>
					{/snippet}
					{#snippet actions()}
						<ActionMenu>
							<DropdownMenu.Item onclick={() => handleShow(selectedItem)}>
								<Eye class="mr-2 size-4" />
								<span>Show in File Manager</span>
								<DropdownMenu.Shortcut>
									{shortcutToText({ key: 'Enter', modifiers: ['cmd'] })}
								</DropdownMenu.Shortcut>
							</DropdownMenu.Item>
							<DropdownMenu.Item onclick={() => handleCopyPath(selectedItem)}>
								<Copy class="mr-2 size-4" />
								<span>Copy Path</span>
								<DropdownMenu.Shortcut>
									{shortcutToText({ key: 'c', modifiers: ['ctrl'] })}
								</DropdownMenu.Shortcut>
							</DropdownMenu.Item>
							<DropdownMenu.Separator />
							<DropdownMenu.Item class="text-red-500" onclick={() => handleDelete(selectedItem)}>
								<Trash class="mr-2 size-4" />
								<span>Move to Trash</span>
								<DropdownMenu.Shortcut>
									{shortcutToText({ key: 'x', modifiers: ['ctrl'] })}
								</DropdownMenu.Shortcut>
							</DropdownMenu.Item>
						</ActionMenu>
					{/snippet}
				</ActionBar>
			{/if}
		</div>
	</div>
</main>
