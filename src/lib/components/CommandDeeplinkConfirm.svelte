<script lang="ts">
	import type { PluginInfo } from '@raycast-linux/protocol';
	import { Button } from './ui/button';
	import Icon from './Icon.svelte';
	import { createEventDispatcher } from 'svelte';
	import path from 'path';

	type Props = {
		plugin: PluginInfo;
	};
	let { plugin }: Props = $props();

	const dispatch = createEventDispatcher<{ confirm: void; cancel: void }>();

	const assetsPath = $derived(path.dirname(plugin.pluginPath) + '/assets');

	function handleConfirm() {
		dispatch('confirm');
	}

	function handleCancel() {
		dispatch('cancel');
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			e.preventDefault();
			handleCancel();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<div
	class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
	onclick={(e) => {
		if (e.target === e.currentTarget) handleCancel();
	}}
	role="dialog"
	aria-modal="true"
	aria-labelledby="dialog-title"
	tabindex={0}
>
	<div
		class="bg-background/80 text-foreground flex w-full max-w-sm flex-col items-center gap-4 rounded-xl border border-white/10 p-6 text-center shadow-2xl"
		role="dialog"
		aria-modal="true"
		aria-labelledby="dialog-title"
	>
		<Icon icon={plugin.icon} class="size-16" {assetsPath} />
		<h2 id="dialog-title" class="text-xl font-semibold">Request to open {plugin.title}</h2>
		<p class="text-muted-foreground text-sm">
			The command was triggered from outside of Raycast. If you did not do this, please cancel the
			operation.
		</p>

		<div class="mt-2 flex w-full flex-col gap-2">
			<Button onclick={handleConfirm} class="w-full text-base" size="lg">Open Command</Button>
			<Button onclick={handleConfirm} variant="secondary" class="w-full text-base" size="lg"
				>Always Open Command</Button
			>
			<Button onclick={handleCancel} variant="ghost" class="w-full text-base" size="lg"
				>Cancel</Button
			>
		</div>
	</div>
</div>
