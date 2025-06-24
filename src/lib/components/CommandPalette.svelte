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
	type FrecencyDataItem = {
		itemId: string;
		useCount: number;
		lastUsedAt: number;
	};

	type Props = {
		plugins: PluginInfo[];
		onRunPlugin: (plugin: PluginInfo) => void;
		installedApps?: App[];
		quicklinks?: Quicklink[];
		frecencyData?: FrecencyDataItem[];
		onItemRun: (itemId: string) => void;
	};

	type UnifiedItem = {
		type: 'calculator' | 'plugin' | 'app' | 'quicklink';
		id: string;
		data: any;
		score: number;
	};

	let {
		plugins,
		onRunPlugin,
		installedApps = [],
		quicklinks = [],
		frecencyData = [],
		onItemRun
	}: Props = $props();

	let searchText = $state('');
	let quicklinkArgument = $state('');
	let selectedIndex = $state(0);
	let listElement: HTMLElement | null = $state(null);
	let searchInputEl: HTMLInputElement | null = $state(null);
	let argumentInputEl: HTMLInputElement | null = $state(null);
	let selectedQuicklinkForArgument: Quicklink | null = $state(null);

	const math = create(all);

	const frecencyMap = $derived(new Map(frecencyData.map((item) => [item.itemId, item])));

	const allSearchableItems = $derived.by(() => {
		const items = [];
		items.push(...plugins.map((p) => ({ type: 'plugin', id: p.pluginPath, data: p }) as const));
		items.push(...installedApps.map((a) => ({ type: 'app', id: a.exec, data: a }) as const));
		items.push(
			...quicklinks.map((q) => ({ type: 'quicklink', id: `quicklink-${q.id}`, data: q }) as const)
		);
		return items;
	});

	const fuse = $derived(
		new Fuse(allSearchableItems, {
			keys: [
				'data.title',
				'data.pluginTitle',
				'data.description',
				'data.name',
				'data.comment',
				'data.link'
			],
			threshold: 0.4,
			includeScore: true
		})
	);

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
		} catch {
			return null;
		}
	});

	const displayItems = $derived.by(() => {
		let items: (UnifiedItem & { fuseScore?: number })[] = [];

		if (searchText.trim()) {
			items = fuse.search(searchText).map((result) => ({
				...result.item,
				score: 0,
				fuseScore: result.score
			}));
		} else {
			items = allSearchableItems.map((item) => ({ ...item, score: 0, fuseScore: 1 }));
		}

		const now = Date.now() / 1000;
		const gravity = 1.8;

		items.forEach((item) => {
			const frecency = frecencyMap.get(item.id);
			let frecencyScore = 0;
			if (frecency) {
				const ageInHours = Math.max(1, (now - frecency.lastUsedAt) / 3600);
				frecencyScore = (frecency.useCount * 1000) / Math.pow(ageInHours + 2, gravity);
			}

			const textScore = item.fuseScore !== undefined ? (1 - item.fuseScore) * 100 : 0;
			item.score = frecencyScore + textScore;
		});

		items.sort((a, b) => b.score - a.score);

		if (calculatorResult) {
			items.unshift({
				type: 'calculator',
				id: 'calculator',
				data: {
					value: searchText,
					result: calculatorResult.value,
					resultType: calculatorResult.type
				},
				score: 9999
			});
		}

		return items;
	});

	$effect(() => {
		const selectedItem = displayItems[selectedIndex];
		if (selectedItem?.type === 'quicklink' && selectedItem.data.link.includes('{argument}')) {
			selectedQuicklinkForArgument = selectedItem.data;
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
		onItemRun(item.id);

		switch (item.type) {
			case 'calculator':
				writeText(item.data.result);
				break;
			case 'plugin':
				onRunPlugin(item.data as PluginInfo);
				break;
			case 'app':
				if (item.data.exec) {
					invoke('launch_app', { exec: item.data.exec }).catch(console.error);
				}
				break;
			case 'quicklink':
				const quicklink = item.data as Quicklink;
				if (quicklink.link.includes('{argument}')) {
					await tick();
					argumentInputEl?.focus();
				} else {
					executeQuicklink(quicklink);
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
</main>
