<script lang="ts">
	import Icon from '../Icon.svelte';
	import { Download } from '@lucide/svelte';
	import type { Datum } from '$lib/store';

	type Props = {
		ext: Datum;
		isSelected: boolean;
		onclick?: () => void;
		onfocus?: () => void;
	};

	let { ext, isSelected, onclick, onfocus }: Props = $props();
</script>

<button
	type="button"
	class="hover:bg-accent/50 flex w-full items-center gap-3 px-4 py-2 text-left"
	class:bg-accent={isSelected}
	{onclick}
	{onfocus}
>
	<Icon
		icon={ext.icons.light ? { source: ext.icons.light, mask: 'Circle' } : undefined}
		class="size-6"
	/>
	<div class="flex-grow overflow-hidden">
		<p class="font-medium">{ext.title}</p>
		<p class="text-muted-foreground truncate text-sm">{ext.description}</p>
	</div>
	<div class="ml-auto flex shrink-0 items-center gap-4">
		{#if ext.commands.length > 0}
			<span class="text-muted-foreground text-sm">{ext.commands.length}</span>
		{/if}
		<div class="text-muted-foreground flex items-center gap-1 text-sm">
			<Download class="size-4" />
			{ext.download_count.toLocaleString()}
		</div>
		<Icon
			icon={ext.author.avatar ? { source: ext.author.avatar, mask: 'Circle' } : undefined}
			class="size-6"
		/>
	</div>
</button>
