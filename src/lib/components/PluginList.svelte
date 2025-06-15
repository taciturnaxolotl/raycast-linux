<script lang="ts">
	import type { PluginInfo } from '@raycast-linux/protocol';
	import Icon from '$lib/components/Icon.svelte';
	import { Input } from '$lib/components/ui/input';

	type Props = {
		plugins: PluginInfo[];
		onRunPlugin: (plugin: PluginInfo) => void;
	};

	let { plugins, onRunPlugin }: Props = $props();

	let searchText = $state('');
	let selectedIndex = $state(0);

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

	$effect(() => {
		if (selectedIndex >= filteredPlugins.length) {
			selectedIndex = Math.max(0, filteredPlugins.length - 1);
		}
	});

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'ArrowDown') {
			event.preventDefault();
			selectedIndex = Math.min(selectedIndex + 1, filteredPlugins.length - 1);
		} else if (event.key === 'ArrowUp') {
			event.preventDefault();
			selectedIndex = Math.max(selectedIndex - 1, 0);
		} else if (event.key === 'Enter') {
			event.preventDefault();
			if (filteredPlugins[selectedIndex]) {
				onRunPlugin(filteredPlugins[selectedIndex]);
			}
		}
	}

	function handleItemClick(index: number) {
		selectedIndex = index;
		onRunPlugin(filteredPlugins[index]);
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
		<div>
			{#each filteredPlugins as plugin, index}
				<button
					type="button"
					class="hover:bg-accent/50 flex w-full items-center gap-3 px-4 py-2 text-left"
					class:bg-accent={selectedIndex === index}
					onclick={() => handleItemClick(index)}
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
		</div>
	</div>
</main>
