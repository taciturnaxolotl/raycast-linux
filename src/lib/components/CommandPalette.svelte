<script lang="ts">
	import type { PluginInfo } from '@raycast-linux/protocol';
	import { Input } from '$lib/components/ui/input';
	import Calculator from '$lib/components/Calculator.svelte';
	import BaseList from '$lib/components/BaseList.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import Fuse from 'fuse.js';
	import ListItemBase from './nodes/shared/ListItemBase.svelte';
	import path from 'path';
	import { create, all } from 'mathjs';
	import { writeText } from '@tauri-apps/plugin-clipboard-manager';
	import type { Quicklink } from '$lib/quicklinks.svelte';
	import { tick } from 'svelte';

	type App = { name: string; comment?: string; exec: string; icon_path?: string };

	type Props = {
		plugins: PluginInfo[];
		onRunPlugin: (plugin: PluginInfo) => void;
		installedApps?: App[];
		quicklinks?: Quicklink[];
	};

	type UnifiedItem =
		| { type: 'calculator'; id: 'calculator'; value: string; result: string; resultType: string }
		| { type: 'plugin'; id: string; data: PluginInfo }
		| { type: 'app'; id: string; data: App }
		| { type: 'quicklink'; id: number; data: Quicklink };

	let { plugins, onRunPlugin, installedApps = [], quicklinks = [] }: Props = $props();

	let searchText = $state('');
	let quicklinkArgument = $state('');
	let selectedIndex = $state(0);
	let listElement: HTMLElement | null = $state(null);
	let searchInputEl: HTMLInputElement | null = $state(null);
	let argumentInputEl: HTMLInputElement | null = $state(null);
	let selectedQuicklinkForArgument: Quicklink | null = $state(null);

	const math = create(all);

	const calculatorResult = $derived.by(() => {
		if (!searchText.trim() || selectedQuicklinkForArgument) {
			return null;
		}

		try {
			const result = math.evaluate(searchText.trim());
			if (typeof result === 'function' || typeof result === 'undefined') return null;
			let resultString = math.format(result, { precision: 14 });
			if (resultString === searchText.trim()) return null;
			return { value: resultString, type: math.typeOf(result) };
		} catch (_error) {
			return null;
		}
	});

	const pluginFuse = $derived(
		new Fuse(plugins, { keys: ['title', 'description', 'pluginName'], threshold: 0.4 })
	);
	const appFuse = $derived(
		new Fuse(installedApps, { keys: ['name', 'comment', 'exec'], threshold: 0.4 })
	);
	const quicklinkFuse = $derived(new Fuse(quicklinks, { keys: ['name', 'link'], threshold: 0.4 }));

	const displayItems = $derived.by(() => {
		const items: UnifiedItem[] = [];

		if (calculatorResult) {
			items.push({
				type: 'calculator',
				id: 'calculator',
				value: searchText,
				result: calculatorResult.value,
				resultType: calculatorResult.type
			});
		}

		const filterAndMap = <T extends { [key: string]: unknown }>(
			data: T[],
			fuse: Fuse<T>,
			type: 'plugin' | 'app' | 'quicklink',
			idKey: keyof T
		) => {
			const filtered = searchText ? fuse.search(searchText) : data.map((item) => ({ item }));
			const unique = [...new Map(filtered.map((res) => [res.item[idKey], res.item])).values()];
			return unique.map((item) => ({ type, id: item[idKey], data: item }));
		};

		items.push(...(filterAndMap(plugins, pluginFuse, 'plugin', 'pluginPath') as UnifiedItem[]));
		items.push(...(filterAndMap(installedApps, appFuse, 'app', 'exec') as UnifiedItem[]));
		items.push(...(filterAndMap(quicklinks, quicklinkFuse, 'quicklink', 'id') as UnifiedItem[]));

		return items as UnifiedItem[];
	});

	$effect(() => {
		const selectedItem = displayItems[selectedIndex];
		if (selectedItem?.type === 'quicklink') {
			selectedQuicklinkForArgument = selectedItem.data;
			searchInputEl?.focus();
		} else {
			selectedQuicklinkForArgument = null;
		}
	});

	function resetState() {
		searchText = '';
		quicklinkArgument = '';
		selectedIndex = 0;
		selectedQuicklinkForArgument = null;
		tick().then(() => searchInputEl?.focus());
	}

	async function executeQuicklink(quicklink: Quicklink, argument?: string) {
		const finalLink = argument
			? quicklink.link.replace(/\{argument\}/g, encodeURIComponent(argument))
			: quicklink.link.replace(/\{argument\}/g, '');
		await invoke('execute_quicklink', {
			link: finalLink,
			application: quicklink.application
		});
		resetState();
	}

	async function handleEnter(item: UnifiedItem) {
		switch (item.type) {
			case 'calculator':
				writeText(item.result);
				break;
			case 'plugin':
				onRunPlugin(item.data);
				break;
			case 'app':
				if (item.data.exec) {
					invoke('launch_app', { exec: item.data.exec }).catch(console.error);
				}
				break;
			case 'quicklink':
				if (item.data.link.includes('{argument}')) {
					await tick();
					argumentInputEl?.focus();
				} else {
					executeQuicklink(item.data);
				}
				break;
		}
	}

	async function handleArgumentKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			if (selectedQuicklinkForArgument) {
				await executeQuicklink(selectedQuicklinkForArgument, quicklinkArgument);
			}
		} else if (e.key === 'Escape' || (e.key === 'Backspace' && quicklinkArgument === '')) {
			e.preventDefault();
			quicklinkArgument = '';
			await tick();
			searchInputEl?.focus();
		}
	}
</script>

<main class="bg-background text-foreground flex h-screen flex-col">
	<header class="flex h-12 shrink-0 items-center border-b px-2">
		<div class="relative flex w-full items-center">
			<Input
				class="w-full rounded-none border-none !bg-transparent pr-0 text-base"
				placeholder={selectedQuicklinkForArgument ? '' : 'Search for extensions and commands...'}
				bind:value={searchText}
				bind:ref={searchInputEl}
				autofocus
			/>

			{#if selectedQuicklinkForArgument}
				<div class="pointer-events-none absolute top-0 left-0 flex h-full w-full items-center">
					<span class="whitespace-pre text-transparent">{searchText}</span>
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
			onenter={handleEnter}
			bind:selectedIndex
			bind:listElement
			autofocus={!selectedQuicklinkForArgument}
		>
			{#snippet itemSnippet({ item, isSelected, onclick })}
				{#if item.type === 'calculator'}
					<Calculator
						searchText={item.value}
						mathResult={item.result}
						mathResultType={item.resultType}
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
</main>
