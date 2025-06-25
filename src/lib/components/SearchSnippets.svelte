<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount, tick, untrack } from 'svelte';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { ArrowLeft, Trash, Loader2 } from '@lucide/svelte';
	import ListItemBase from './nodes/shared/ListItemBase.svelte';
	import { Kbd } from './ui/kbd';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { shortcutToText } from '$lib/renderKey';
	import ActionBar from './nodes/shared/ActionBar.svelte';
	import ActionMenu from './nodes/shared/ActionMenu.svelte';
	import BaseList from './BaseList.svelte';

	type Props = {
		onBack: () => void;
	};

	type Snippet = {
		id: number;
		name: string;
		keyword: string;
		content: string;
		createdAt: string;
		updatedAt: string;
		timesUsed: number;
		lastUsedAt: string;
	};

	type DisplayItem = {
		id: number | string;
		itemType: 'item' | 'header';
		data: Snippet | string;
	};

	let { onBack }: Props = $props();

	let snippets = $state<Snippet[]>([]);
	let selectedIndex = $state(0);
	let searchText = $state('');
	let isFetching = $state(false);

	const displayedItems = $derived.by(() => {
		const items: DisplayItem[] = [];
		let lastHeader = '';

		const today = new Date();
		today.setHours(0, 0, 0, 0);
		const yesterday = new Date(today);
		yesterday.setDate(yesterday.getDate() - 1);

		for (const snippet of snippets) {
			const snippetDate = new Date(snippet.updatedAt);

			let header = '';
			const snippetDay = new Date(snippetDate);
			snippetDay.setHours(0, 0, 0, 0);

			if (snippetDay.getTime() === today.getTime()) {
				header = 'Today';
			} else if (snippetDay.getTime() === yesterday.getTime()) {
				header = 'Yesterday';
			} else {
				header = snippetDay.toLocaleDateString(undefined, {
					year: 'numeric',
					month: 'long'
				});
			}

			if (header !== lastHeader) {
				items.push({ id: `header-${header}`, itemType: 'header', data: header });
				lastHeader = header;
			}
			items.push({ id: snippet.id, itemType: 'item', data: snippet });
		}
		return items;
	});

	const selectedItem = $derived(
		displayedItems[selectedIndex]?.itemType === 'item'
			? (displayedItems[selectedIndex].data as Snippet)
			: null
	);

	const fetchSnippets = async () => {
		if (isFetching) return;
		isFetching = true;
		try {
			const newItems = await invoke<Snippet[]>('list_snippets', {
				searchTerm: searchText || null
			});
			snippets = newItems;
			if (selectedIndex >= newItems.length) {
				selectedIndex = 0;
			}
		} catch (e) {
			console.error('Failed to fetch snippets:', e);
		} finally {
			isFetching = false;
		}
	};

	const formatDateTime = (dateString: string) => {
		const date = new Date(dateString);
		if (date.getFullYear() < 1971) return 'Never';
		return `Today at ${date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' })}`;
	};

	const handlePaste = async (item: Snippet) => {
		await invoke('paste_snippet_content', { content: item.content });
		await invoke('snippet_was_used', { id: item.id });
		const updatedItems = snippets.map((i) =>
			i.id === item.id
				? {
						...i,
						timesUsed: i.timesUsed + 1,
						lastUsedAt: new Date().toISOString()
					}
				: i
		);
		snippets = updatedItems;
		onBack();
	};

	const handleDelete = async (item: Snippet) => {
		await invoke('delete_snippet', { id: item.id });
		fetchSnippets();
	};

	const handleKeydown = (e: KeyboardEvent) => {
		if (e.key === 'Escape') {
			e.preventDefault();
			onBack();
			return;
		}
		if (!selectedItem) return;

		if (e.ctrlKey && e.key.toLowerCase() === 'x') {
			e.preventDefault();
			handleDelete(selectedItem);
		}
	};

	onMount(() => {
		fetchSnippets();
	});

	$effect(() => {
		const term = searchText;
		untrack(() => {
			const timer = setTimeout(() => {
				if (term === searchText) {
					fetchSnippets();
				}
			}, 300);
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
			placeholder="Search snippets..."
			bind:value={searchText}
			autofocus
		/>
	</header>
	<div class="grid grow grid-cols-[minmax(0,_1.5fr)_minmax(0,_2.5fr)] overflow-y-hidden">
		<div class="flex-grow overflow-y-auto border-r">
			{#if isFetching && snippets.length === 0}
				<div class="text-muted-foreground flex h-full items-center justify-center">
					<Loader2 class="size-6 animate-spin" />
				</div>
			{:else}
				<BaseList
					items={displayedItems}
					bind:selectedIndex
					onenter={(item) => handlePaste(item.data as Snippet)}
					isItemSelectable={(item) => item.itemType === 'item'}
				>
					{#snippet itemSnippet({ item, isSelected, onclick: itemOnClick })}
						{#if item.itemType === 'header'}
							<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">
								{item.data as string}
							</h3>
						{:else if item.itemType === 'item'}
							{@const snippetItem = item.data as Snippet}
							<button class="w-full text-left" onclick={itemOnClick}>
								<ListItemBase
									icon="snippets-16"
									title={snippetItem.name}
									subtitle={snippetItem.keyword}
									{isSelected}
								/>
							</button>
						{/if}
					{/snippet}
				</BaseList>
			{/if}
		</div>
		<div class="flex flex-col overflow-y-hidden">
			{#if selectedItem}
				<div class="relative flex-grow overflow-y-auto p-4">
					<div class="font-mono text-sm whitespace-pre-wrap">{selectedItem.content}</div>
				</div>

				<div class="border-t p-4">
					<h3 class="text-muted-foreground mb-2 text-xs font-semibold uppercase">Information</h3>
					<div class="flex flex-col gap-3 text-sm">
						<div class="flex justify-between">
							<span class="text-muted-foreground">Name</span>
							<span>{selectedItem.name}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-muted-foreground">Content type</span>
							<span class="capitalize">Text</span>
						</div>
						<div class="flex justify-between">
							<span class="text-muted-foreground">Times used</span>
							<span>{selectedItem.timesUsed}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-muted-foreground">Last used</span>
							<span>{formatDateTime(selectedItem.lastUsedAt)}</span>
						</div>
					</div>
				</div>

				<ActionBar>
					{#snippet primaryAction({ props })}
						<Button {...props} onclick={() => handlePaste(selectedItem)}>
							Paste <Kbd>⏎</Kbd>
						</Button>
					{/snippet}
					{#snippet actions()}
						<ActionMenu>
							<DropdownMenu.Item onclick={() => handleDelete(selectedItem)}>
								<Trash class="mr-2 size-4" />
								<span>Delete</span>
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
