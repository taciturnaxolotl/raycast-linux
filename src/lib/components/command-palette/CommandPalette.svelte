<script lang="ts">
	import type { PluginInfo } from '@raycast-linux/protocol';
	import { Input } from '$lib/components/ui/input';
	import Calculator from '$lib/components/Calculator.svelte';
	import BaseList from '$lib/components/BaseList.svelte';
	import ListItemBase from '../nodes/shared/ListItemBase.svelte';
	import path from 'path';
	import { tick } from 'svelte';
	import type { Quicklink } from '$lib/quicklinks.svelte';
	import { appsStore } from '$lib/apps.svelte';
	import { frecencyStore } from '$lib/frecency.svelte';
	import { quicklinksStore } from '$lib/quicklinks.svelte';
	import { useCommandPaletteItems, useCommandPaletteActions } from '$lib/command-palette.svelte';
	import CommandPaletteActionBar from './ActionBar.svelte';

	type Props = {
		plugins: PluginInfo[];
		onRunPlugin: (plugin: PluginInfo) => void;
	};

	let { plugins, onRunPlugin }: Props = $props();

	const { apps: installedApps } = $derived(appsStore);
	const { quicklinks } = $derived(quicklinksStore);
	const { data: frecencyData } = $derived(frecencyStore);

	let searchText = $state('');
	let quicklinkArgument = $state('');
	let selectedIndex = $state(0);
	let listElement: HTMLElement | null = $state(null);
	let searchInputEl: HTMLInputElement | null = $state(null);
	let argumentInputEl: HTMLInputElement | null = $state(null);
	let selectedQuicklinkForArgument: Quicklink | null = $state(null);

	const { displayItems } = $derived.by(
		useCommandPaletteItems({
			searchText: () => searchText,
			plugins: () => plugins,
			installedApps: () => installedApps,
			quicklinks: () => quicklinks,
			frecencyData: () => frecencyData,
			selectedQuicklinkForArgument: () => selectedQuicklinkForArgument
		})
	);

	const selectedItem = $derived(displayItems[selectedIndex]);

	function resetState() {
		searchText = '';
		quicklinkArgument = '';
		selectedIndex = 0;
		selectedQuicklinkForArgument = null;
		tick().then(() => searchInputEl?.focus());
	}

	async function focusArgumentInput() {
		await tick();
		argumentInputEl?.focus();
	}

	const actions = useCommandPaletteActions({
		selectedItem: () => selectedItem,
		onRunPlugin,
		resetState,
		focusArgumentInput
	});

	$effect(() => {
		const item = displayItems[selectedIndex];
		if (item?.type === 'quicklink' && item.data.link.includes('{argument}')) {
			selectedQuicklinkForArgument = item.data;
		} else {
			selectedQuicklinkForArgument = null;
		}
	});

	async function handleArgumentKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			if (selectedQuicklinkForArgument) {
				await actions.executeQuicklink(selectedQuicklinkForArgument, quicklinkArgument);
			}
		} else if (e.key === 'Escape' || (e.key === 'Backspace' && quicklinkArgument === '')) {
			e.preventDefault();
			quicklinkArgument = '';
			await tick();
			searchInputEl?.focus();
		}
	}

	async function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Escape' && searchText) {
			e.preventDefault();
			searchText = '';
			await tick();
			searchInputEl?.focus();
		}

		if (!selectedItem) return;

		const keyMap = {
			'C-S-c': selectedItem.type === 'plugin' ? actions.handleCopyDeeplink : undefined,
			'C-S-,': selectedItem.type === 'plugin' ? actions.handleConfigureCommand : undefined,
			'C-.': selectedItem.type === 'app' ? actions.handleCopyAppName : undefined,
			'C-S-.': selectedItem.type === 'app' ? actions.handleCopyAppPath : undefined,
			'C-h': selectedItem.type === 'app' ? actions.handleHideApp : undefined
		};

		const shortcut = `${e.metaKey ? 'M-' : ''}${e.ctrlKey ? 'C-' : ''}${e.shiftKey ? 'S-' : ''}${e.key.toLowerCase()}`;
		const action = keyMap[shortcut as keyof typeof keyMap];

		if (action) {
			e.preventDefault();
			await action();
		}
	}
</script>

<main class="bg-background text-foreground flex h-screen flex-col">
	<header class="flex h-12 shrink-0 items-center border-b px-2">
		<div class="relative flex w-full items-center">
			<Input
				class="w-full rounded-none border-none !bg-transparent pr-0 text-base"
				placeholder={selectedQuicklinkForArgument
					? selectedQuicklinkForArgument.name
					: 'Search for extensions and commands...'}
				bind:value={searchText}
				bind:ref={searchInputEl}
				onkeydown={handleKeyDown}
				autofocus
			/>

			{#if selectedQuicklinkForArgument}
				<div class="pointer-events-none absolute top-0 left-0 flex h-full w-full items-center">
					<span class="whitespace-pre text-transparent"
						>{searchText || selectedQuicklinkForArgument.name}</span
					>
					<span class="w-2"></span>
					<div class="pointer-events-auto">
						<div class="inline-grid items-center">
							<span
								class="invisible col-start-1 row-start-1 px-3 text-base whitespace-pre md:text-sm"
								aria-hidden="true"
							>
								{quicklinkArgument || 'Query'}
							</span>

							<Input
								class="col-start-1 row-start-1 h-7 w-full"
								placeholder="Query"
								bind:value={quicklinkArgument}
								bind:ref={argumentInputEl}
								onkeydown={handleArgumentKeydown}
							/>
						</div>
					</div>
				</div>
			{/if}
		</div>
	</header>

	<div class="grow overflow-y-auto">
		<BaseList
			items={displayItems.map((item) => ({ ...item, itemType: 'item' }))}
			onenter={actions.handleEnter}
			bind:selectedIndex
			bind:listElement
		>
			{#snippet itemSnippet({ item, isSelected, onclick })}
				{#if item.type === 'calculator'}
					<Calculator
						searchText={item.data.value}
						mathResult={item.data.result}
						mathResultType={item.data.resultType}
						{isSelected}
						onSelect={onclick}
					/>
				{:else if item.type === 'plugin'}
					{@const assetsPath = path.dirname(item.data.pluginPath) + '/assets'}
					<ListItemBase
						title={item.data.title}
						subtitle={item.data.pluginTitle}
						icon={item.data.icon || 'app-window-16'}
						{assetsPath}
						{isSelected}
						{onclick}
					>
						{#snippet accessories()}
							<span class="text-muted-foreground ml-auto text-xs whitespace-nowrap"> Command </span>
						{/snippet}
					</ListItemBase>
				{:else if item.type === 'app'}
					<ListItemBase
						title={item.data.name}
						subtitle={item.data.comment}
						icon={item.data.icon_path ?? 'app-window-16'}
						{isSelected}
						{onclick}
					>
						{#snippet accessories()}
							<span class="text-muted-foreground ml-auto text-xs whitespace-nowrap">
								Application
							</span>
						{/snippet}
					</ListItemBase>
				{:else if item.type === 'quicklink'}
					<ListItemBase
						title={item.data.name}
						subtitle={item.data.link.replace(/\{argument\}/g, '...')}
						icon={item.data.icon ?? 'link-16'}
						{isSelected}
						{onclick}
					>
						{#snippet accessories()}
							<span class="text-muted-foreground ml-auto text-xs whitespace-nowrap">
								Quicklink
							</span>
						{/snippet}
					</ListItemBase>
				{/if}
			{/snippet}
		</BaseList>
	</div>

	<CommandPaletteActionBar {selectedItem} {actions} />
</main>
