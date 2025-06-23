<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { ArrowLeft, Pin, Trash } from '@lucide/svelte';
	import ListItemBase from './nodes/shared/ListItemBase.svelte';
	import { convertFileSrc } from '@tauri-apps/api/core';
	import { writeText } from '@tauri-apps/plugin-clipboard-manager';
	import Fuse from 'fuse.js';
	import { Kbd } from './ui/kbd';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { shortcutToText } from '$lib/renderKey';
	import * as Select from './ui/select';
	import ActionBar from './nodes/shared/ActionBar.svelte';
	import ActionMenu from './nodes/shared/ActionMenu.svelte';
	import BaseList from './BaseList.svelte';

	type Props = {
		onBack: () => void;
	};

	let { onBack }: Props = $props();

	type ClipboardItem = {
		id: number;
		hash: string;
		contentType: 'text' | 'image' | 'color' | 'link' | 'file';
		contentValue: string;
		sourceAppName: string | null;
		firstCopiedAt: string;
		lastCopiedAt: string;
		timesCopied: number;
		isPinned: boolean;
	};

	type DisplayItem = {
		id: number | string;
		itemType: 'item' | 'header';
		data: ClipboardItem | string;
	};

	let allItems = $state<ClipboardItem[]>([]);
	let selectedIndex = $state(0);
	let searchText = $state('');
	let filter = $state('all');

	const fuse = $derived(
		new Fuse(
			allItems.filter((item) => !item.isPinned),
			{
				keys: ['contentValue'],
				threshold: 0.3
			}
		)
	);

	const displayedItems = $derived.by(() => {
		const items: DisplayItem[] = [];
		const pinned = allItems.filter((item) => item.isPinned);
		const recent = searchText
			? fuse.search(searchText).map((r) => r.item)
			: allItems.filter((item) => !item.isPinned);

		if (pinned.length > 0) {
			items.push({ id: 'header-pinned', itemType: 'header', data: 'Pinned' });
			items.push(...pinned.map((item) => ({ id: item.id, itemType: 'item' as const, data: item })));
		}
		if (recent.length > 0) {
			items.push({ id: 'header-recent', itemType: 'header', data: 'Most Recent' });
			items.push(...recent.map((item) => ({ id: item.id, itemType: 'item' as const, data: item })));
		}
		return items;
	});

	const selectedItem = $derived(
		displayedItems[selectedIndex]?.itemType === 'item'
			? (displayedItems[selectedIndex].data as ClipboardItem)
			: null
	);

	async function fetchHistory() {
		try {
			const items = await invoke<ClipboardItem[]>('history_get_items', {
				filter,
				limit: 200
			});
			allItems = items;
			selectedIndex = Math.min(selectedIndex, displayedItems.length - 1);
		} catch (e) {
			console.error('Failed to fetch clipboard history:', e);
		}
	}

	onMount(() => {
		fetchHistory();
	});

	$effect(() => {
		fetchHistory();
	});

	function getIconForType(type: ClipboardItem['contentType']): string {
		switch (type) {
			case 'text':
				return 'text-16';
			case 'image':
				return 'image-16';
			case 'color':
				return 'swatch-16';
			case 'link':
				return 'link-16';
			case 'file':
				return 'blank-document-16';
			default:
				return 'question-mark-circle-16';
		}
	}

	function formatDateTime(dateString: string) {
		const date = new Date(dateString);
		return `Today at ${date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' })}`;
	}

	async function handleCopy(item: ClipboardItem) {
		await writeText(item.contentValue);
		await invoke('history_item_was_copied', { id: item.id });
		await fetchHistory();
	}

	async function handlePin(item: ClipboardItem) {
		await invoke('history_toggle_pin', { id: item.id });
		await fetchHistory();
	}

	async function handleDelete(item: ClipboardItem) {
		await invoke('history_delete_item', { id: item.id });
		await fetchHistory();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			e.preventDefault();
			onBack();
			return;
		}
		if (!selectedItem) return;

		if (e.metaKey && e.shiftKey && e.key.toLowerCase() === 'p') {
			e.preventDefault();
			handlePin(selectedItem);
		}
		if (e.ctrlKey && e.key.toLowerCase() === 'x') {
			e.preventDefault();
			handleDelete(selectedItem);
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<main class="bg-background text-foreground flex h-screen flex-col">
	<header class="flex h-12 shrink-0 items-center border-b px-2">
		<Button variant="ghost" size="icon" onclick={onBack}>
			<ArrowLeft class="size-5" />
		</Button>
		<Input
			class="rounded-none border-none !bg-transparent pr-0"
			placeholder="Type to filter entries..."
			bind:value={searchText}
			autofocus
		/>
		<Select.Root bind:value={filter} type="single">
			<Select.Trigger class="w-32">
				{filter ?? 'All Types'}
			</Select.Trigger>
			<Select.Content>
				<Select.Item value="all">All Types</Select.Item>
				<Select.Item value="text">Text</Select.Item>
				<Select.Item value="image">Images</Select.Item>
				<Select.Item value="link">Links</Select.Item>
				<Select.Item value="color">Colors</Select.Item>
			</Select.Content>
		</Select.Root>
	</header>
	<div class="grid grow grid-cols-[minmax(0,_1.5fr)_minmax(0,_2.5fr)] overflow-y-hidden">
		<div class="flex-grow overflow-y-auto border-r">
			<BaseList
				items={displayedItems}
				bind:selectedIndex
				onenter={(item) => handleCopy(item.data as ClipboardItem)}
				isItemSelectable={(item) => item.itemType === 'item'}
			>
				{#snippet itemSnippet({ item, isSelected, onclick })}
					{#if item.itemType === 'header'}
						<h3 class="text-muted-foreground px-4 pt-2.5 pb-1 text-xs font-semibold uppercase">
							{item.data as string}
						</h3>
					{:else if item.itemType === 'item'}
						{@const clipboardItem = item.data as ClipboardItem}
						<button class="w-full" {onclick}>
							<ListItemBase
								icon={getIconForType(clipboardItem.contentType)}
								title={clipboardItem.contentValue}
								{isSelected}
							/>
						</button>
					{/if}
				{/snippet}
			</BaseList>
		</div>
		<div class="flex flex-col overflow-y-hidden">
			{#if selectedItem}
				<div class="flex-grow overflow-y-auto p-4">
					{#if selectedItem.contentType === 'color'}
						<div class="flex flex-col items-center justify-center gap-4 py-8">
							<div
								class="size-24 rounded-full border"
								style:background-color={selectedItem.contentValue}
							></div>
							<p class="font-mono text-lg">{selectedItem.contentValue}</p>
						</div>
					{:else if selectedItem.contentType === 'image'}
						<img
							src={convertFileSrc(selectedItem.contentValue)}
							alt="Clipboard content"
							class="max-h-60 w-full rounded-lg object-contain"
						/>
					{:else}
						<div class="rounded bg-black/10 p-4 font-mono text-sm whitespace-pre-wrap">
							{selectedItem.contentValue}
						</div>
					{/if}

					<div class="mt-4 border-t py-4">
						<h3 class="text-muted-foreground mb-2 text-xs font-semibold uppercase">Information</h3>
						<div class="flex flex-col gap-3 text-sm">
							<div class="flex justify-between">
								<span class="text-muted-foreground">Application</span>
								<span>{selectedItem.sourceAppName ?? 'Unknown'}</span>
							</div>
							<div class="flex justify-between">
								<span class="text-muted-foreground">Content type</span>
								<span class="capitalize">{selectedItem.contentType}</span>
							</div>
							<div class="flex justify-between">
								<span class="text-muted-foreground">Times copied</span>
								<span>{selectedItem.timesCopied}</span>
							</div>
							<div class="flex justify-between">
								<span class="text-muted-foreground">Last copied</span>
								<span>{formatDateTime(selectedItem.lastCopiedAt)}</span>
							</div>
							<div class="flex justify-between">
								<span class="text-muted-foreground">First copied</span>
								<span>{formatDateTime(selectedItem.firstCopiedAt)}</span>
							</div>
						</div>
					</div>
				</div>

				<ActionBar>
					{#snippet primaryAction({ props })}
						<Button {...props} onclick={() => handleCopy(selectedItem)}>
							Copy to Clipboard <Kbd>⏎</Kbd>
						</Button>
					{/snippet}
					{#snippet actions()}
						<ActionMenu>
							<DropdownMenu.Item onclick={() => handlePin(selectedItem)}>
								<Pin class="mr-2 size-4" />
								<span>{selectedItem.isPinned ? 'Unpin' : 'Pin'}</span>
								<DropdownMenu.Shortcut>
									{shortcutToText({ key: 'P', modifiers: ['cmd', 'shift'] })}
								</DropdownMenu.Shortcut>
							</DropdownMenu.Item>
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
