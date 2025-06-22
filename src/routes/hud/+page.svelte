<script lang="ts">
	import { page } from '$app/state';
	import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';

	let hudText = $derived(page.url.searchParams.get('title') ?? '');
	let hudEl = $state<HTMLDivElement | null>(null);

	$effect(() => {
		const bounds = hudEl?.getBoundingClientRect();
		if (bounds) {
			const window = getCurrentWindow();
			window.setMinSize(new LogicalSize(bounds.width, bounds.height));
			window.setMaxSize(new LogicalSize(bounds.width, bounds.height));
			window.setSize(new LogicalSize(bounds.width, bounds.height));
			window.center();
		}
	});
</script>

<div class="flex h-screen items-center justify-center bg-transparent">
	{#if hudText}
		<div
			class="rounded-full bg-black/70 px-4 py-2 text-sm font-medium text-white"
			bind:this={hudEl}
		>
			{hudText}
		</div>
	{/if}
</div>

<style>
	:global(body) {
		background-color: transparent;
	}
</style>
