<script lang="ts">
	import Icon from '../Icon.svelte';
	import { Download } from '@lucide/svelte';
	import type { Datum } from '$lib/store';
	import ListItemBase from '../nodes/shared/ListItemBase.svelte';

	type Props = {
		ext: Datum;
		isSelected: boolean;
		onclick?: () => void;
		onfocus?: () => void;
	};

	let { ext, isSelected, onclick, onfocus }: Props = $props();
</script>

<ListItemBase
	title={ext.title}
	subtitle={ext.description}
	icon={ext.icons.light ? { source: ext.icons.light, mask: 'Circle' } : undefined}
	{isSelected}
	{onclick}
>
	{#snippet accessories()}
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
	{/snippet}
</ListItemBase>
