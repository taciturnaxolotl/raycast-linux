<script lang="ts">
	import type { PluginInfo } from '@raycast-linux/protocol';
	import { Input } from '$lib/components/ui/input';
	import Calculator from '$lib/components/Calculator.svelte';
	import BaseList from '$lib/components/BaseList.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import Fuse from 'fuse.js';
	import ListItemBase from './nodes/shared/ListItemBase.svelte';
	import path from 'path';

	type Props = {
		plugins: PluginInfo[];
		onRunPlugin: (plugin: PluginInfo) => void;
		installedApps?: any[];
	};

	type UnifiedItem =
		| { type: 'calculator'; id: 'calculator'; value: string }
		| { type: 'plugin'; id: string; data: PluginInfo }
		| { type: 'app'; id: string; data: any };

	let { plugins, onRunPlugin, installedApps = [] }: Props = $props();

	let searchText = $state('');
	let calculator: Calculator | null = $state(null);

	const pluginFuse = $derived(
		new Fuse(plugins, {
			keys: [
				{ name: 'title', weight: 0.7 },
				{ name: 'description', weight: 0.2 },
				{ name: 'pluginName', weight: 0.1 }
			],
			threshold: 0.4
		})
	);

	const appFuse = $derived(
		new Fuse(installedApps, {
			keys: ['name', 'comment', 'exec'],
			threshold: 0.4
		})
	);

	const displayItems = $derived.by(() => {
		const filteredPlugins = searchText
			? pluginFuse.search(searchText)
			: plugins.map((p) => ({ item: p }));
		const uniquePlugins = [...new Map(filteredPlugins.map((p) => [p.item.pluginPath, p])).values()];

		const filteredApps = searchText
			? appFuse.search(searchText)
			: installedApps.map((a) => ({ item: a }));
		const uniqueApps = [...new Map(filteredApps.map((a) => [a.item.exec, a])).values()];

		return [
			{ type: 'calculator', id: 'calculator', value: searchText } as const,
			...uniquePlugins.map(
				(p) => ({ type: 'plugin', id: p.item.pluginPath, data: p.item }) as const
			),
			...uniqueApps.map((a) => ({ type: 'app', id: a.item.exec, data: a.item }) as const)
		];
	});

	function handleEnter(item: UnifiedItem) {
		switch (item.type) {
			case 'calculator':
				calculator?.handleClick();
				break;
			case 'plugin':
				onRunPlugin(item.data);
				break;
			case 'app':
				if (item.data.exec) {
					invoke('launch_app', { exec: item.data.exec }).catch(console.error);
				}
				break;
		}
	}
</script>

<main class="bg-background text-foreground flex h-screen flex-col">
	<header class="flex h-12 shrink-0 items-center border-b px-2">
		<Input
			class="rounded-none border-none !bg-transparent pr-0"
			placeholder="Search for extensions and commands..."
			bind:value={searchText}
			autofocus
		/>
	</header>
	<div class="grow overflow-y-auto">
		<BaseList items={displayItems} onenter={handleEnter}>
			{#snippet itemSnippet({ item, isSelected, onclick })}
				{#if item.type === 'calculator'}
					<Calculator
						bind:this={calculator}
						searchText={item.value}
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
						subtitle={item.data.comment || 'No description'}
						icon={item.data.icon_path ? item.data.icon_path : 'app-window-16'}
						{isSelected}
						{onclick}
					>
						{#snippet accessories()}
							<span class="text-muted-foreground ml-auto text-xs whitespace-nowrap">
								Application
							</span>
						{/snippet}
					</ListItemBase>
				{/if}
			{/snippet}
		</BaseList>
	</div>
</main>
