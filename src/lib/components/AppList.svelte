<script lang="ts">
	import Icon from '$lib/components/Icon.svelte';
	import { convertFileSrc } from '@tauri-apps/api/core';
	import { invoke } from '@tauri-apps/api/core';

	type Props = {
		apps: any[];
		searchText: string;
		selectedIndex: number;
		startIndex: number;
		onItemClick: (index: number) => void;
	};

	let { apps, searchText, selectedIndex, startIndex, onItemClick }: Props = $props();

	const filteredApps = $derived.by(() => {
		if (!searchText) return apps;
		const lowerCaseSearch = searchText.toLowerCase();
		return apps.filter(
			(app: any) =>
				app.name.toLowerCase().includes(lowerCaseSearch) ||
				app.comment?.toLowerCase().includes(lowerCaseSearch) ||
				app.exec.toLowerCase().includes(lowerCaseSearch)
		);
	});

	export function getFilteredApps() {
		return filteredApps;
	}

	function handleClick(index: number) {
		const absoluteIndex = startIndex + index;
		onItemClick(absoluteIndex);
		const app = filteredApps[index];
		if (app && app.exec) {
			invoke('launch_app', { exec: app.exec }).catch(console.error);
		}
	}
</script>

{#each filteredApps as app, index}
	{@const absoluteIndex = startIndex + index}
	<button
		type="button"
		class="hover:bg-accent/50 flex w-full items-center gap-3 px-4 py-2 text-left"
		class:bg-accent={selectedIndex === absoluteIndex}
		onclick={() => handleClick(index)}
	>
		<div class="flex size-5 shrink-0 items-center justify-center">
			{#if app.icon_path}
				<img src={convertFileSrc(app.icon_path)} alt="" class="size-4" />
			{:else}
				<Icon icon="app-window-16" class="size-4" />
			{/if}
		</div>
		<div class="flex flex-col">
			<span class="font-medium">{app.name}</span>
			<span class="text-muted-foreground text-sm">{app.comment || 'No description'}</span>
		</div>
		<span class="ml-auto text-xs text-gray-500">System App</span>
	</button>
{/each}
