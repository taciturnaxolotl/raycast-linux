<script lang="ts">
	import type { PluginInfo } from '@raycast-linux/protocol';
	import Icon from '$lib/components/Icon.svelte';

	type Props = {
		plugins: PluginInfo[];
		searchText: string;
		selectedIndex: number;
		startIndex: number;
		onItemClick: (index: number) => void;
		onRunPlugin: (plugin: PluginInfo) => void;
	};

	let { plugins, searchText, selectedIndex, startIndex, onItemClick, onRunPlugin }: Props =
		$props();

	const filteredPlugins = $derived.by(() => {
		if (!searchText) return plugins;
		const lowerCaseSearch = searchText.toLowerCase();
		return plugins.filter(
			(p: PluginInfo) =>
				p.title.toLowerCase().includes(lowerCaseSearch) ||
				p.description?.toLowerCase().includes(lowerCaseSearch) ||
				p.pluginName.toLowerCase().includes(lowerCaseSearch)
		);
	});

	export function getFilteredPlugins() {
		return filteredPlugins;
	}

	function handleClick(index: number) {
		const absoluteIndex = startIndex + index;
		onItemClick(absoluteIndex);
		onRunPlugin(filteredPlugins[index]);
	}
</script>

{#each filteredPlugins as plugin, index}
	{@const absoluteIndex = startIndex + index}
	<button
		type="button"
		class="hover:bg-accent/50 flex w-full items-center gap-3 px-4 py-2 text-left"
		class:bg-accent={selectedIndex === absoluteIndex}
		onclick={() => handleClick(index)}
	>
		<div class="flex size-5 shrink-0 items-center justify-center">
			<Icon icon="app-window-16" class="size-4" />
		</div>
		<div class="flex flex-col">
			<span class="font-medium">{plugin.title}</span>
			<span class="text-muted-foreground text-sm">{plugin.description}</span>
		</div>
		<span class="ml-auto text-xs text-gray-500">{plugin.pluginName}</span>
	</button>
{/each}
