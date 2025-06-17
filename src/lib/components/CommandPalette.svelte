<script lang="ts">
	import type { PluginInfo } from '@raycast-linux/protocol';
	import { Input } from '$lib/components/ui/input';
	import Calculator from '$lib/components/Calculator.svelte';
	import PluginListItems from '$lib/components/PluginListItems.svelte';
	import AppList from '$lib/components/AppList.svelte';

	type Props = {
		plugins: PluginInfo[];
		onRunPlugin: (plugin: PluginInfo) => void;
		installedApps?: any[];
	};

	let { plugins, onRunPlugin, installedApps = [] }: Props = $props();

	let searchText = $state('');
	let selectedIndex = $state(0);
	let calculator: Calculator;
	let pluginList: PluginListItems;
	let appList: AppList;

	$effect(() => {
		if (calculator?.getHasResult()) {
			selectedIndex = 0;
		}
	});

	$effect(() => {
		const calculatorCount = calculator?.getHasResult() ? 1 : 0;
		const pluginCount = pluginList?.getFilteredPlugins()?.length || 0;
		const appCount = appList?.getFilteredApps()?.length || 0;
		const totalItems = calculatorCount + pluginCount + appCount;

		if (selectedIndex >= totalItems) {
			selectedIndex = Math.max(0, totalItems - 1);
		}
	});

	function handleKeydown(event: KeyboardEvent) {
		const calculatorCount = calculator?.getHasResult() ? 1 : 0;
		const pluginCount = pluginList?.getFilteredPlugins()?.length || 0;
		const appCount = appList?.getFilteredApps()?.length || 0;
		const totalItems = calculatorCount + pluginCount + appCount;

		if (totalItems === 0) return;

		if (event.key === 'ArrowDown') {
			event.preventDefault();
			selectedIndex = (selectedIndex + 1) % totalItems;
		} else if (event.key === 'ArrowUp') {
			event.preventDefault();
			selectedIndex = (selectedIndex - 1 + totalItems) % totalItems;
		} else if (event.key === 'Enter') {
			event.preventDefault();
			handleEnter();
		}
	}

	function handleEnter() {
		const calculatorCount = calculator?.getHasResult() ? 1 : 0;
		const pluginCount = pluginList?.getFilteredPlugins()?.length || 0;

		if (calculatorCount && selectedIndex === 0) {
			calculator?.handleClick?.();
		} else {
			const itemIndex = selectedIndex - calculatorCount;
			if (itemIndex < pluginCount) {
				const filteredPlugins = pluginList?.getFilteredPlugins() || [];
				if (filteredPlugins[itemIndex]) {
					onRunPlugin(filteredPlugins[itemIndex]);
				}
			} else {
				const appIndex = itemIndex - pluginCount;
				const filteredApps = appList?.getFilteredApps() || [];
				if (filteredApps[appIndex]) {
					const app = filteredApps[appIndex];
					if (app.exec) {
						import('@tauri-apps/api/core').then(({ invoke }) => {
							invoke('launch_app', { exec: app.exec }).catch(console.error);
						});
					}
				}
			}
		}
	}

	function handleItemClick(index: number) {
		selectedIndex = index;
	}
</script>

<svelte:window onkeydown={handleKeydown} />

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
		<Calculator
			bind:this={calculator}
			{searchText}
			isSelected={selectedIndex === 0}
			onSelect={() => handleItemClick(0)}
		/>
		<div>
			<PluginListItems
				bind:this={pluginList}
				{plugins}
				{searchText}
				{selectedIndex}
				startIndex={calculator?.getHasResult() ? 1 : 0}
				onItemClick={handleItemClick}
				{onRunPlugin}
			/>
			<AppList
				bind:this={appList}
				apps={installedApps}
				{searchText}
				{selectedIndex}
				startIndex={(calculator?.getHasResult() ? 1 : 0) +
					(pluginList?.getFilteredPlugins()?.length || 0)}
				onItemClick={handleItemClick}
			/>
		</div>
	</div>
</main>
