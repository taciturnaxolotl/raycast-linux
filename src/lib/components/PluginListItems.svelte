<script lang="ts">
	import Fuse from 'fuse.js';
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

	const fuse = $derived(
		new Fuse(plugins, {
			keys: [
				{ name: 'title', weight: 0.7 },
				{ name: 'description', weight: 0.2 },
				{ name: 'pluginName', weight: 0.1 }
			],
			threshold: 0.4
		})
	);

	const filteredPlugins = $derived.by(() => {
		if (!searchText) return plugins;
		return fuse.search(searchText).map((result) => result.item);
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
		<span class="text-muted-foreground ml-auto text-xs whitespace-nowrap">{plugin.pluginName}</span>
	</button>
{/each}
