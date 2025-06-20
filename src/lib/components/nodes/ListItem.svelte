<script lang="ts">
	import type { ListItemProps } from '$lib/props';
	import type { HTMLButtonAttributes } from 'svelte/elements';
	import ListItemBase from './shared/ListItemBase.svelte';

	type Props = {
		props: ListItemProps;
		selected: boolean;
	} & HTMLButtonAttributes;

	let { props, selected, ...restProps }: Props = $props();
</script>

<ListItemBase
	title={props.title}
	icon={props.icon}
	isSelected={selected}
	onclick={restProps.onclick}
>
	{#if props.accessories && props.accessories.length > 0}
		{#snippet accessories()}
			{#each props.accessories ?? [] as accessory}
				<span class="text-muted-foreground text-sm">{accessory.text}</span>
			{/each}
		{/snippet}
	{/if}
</ListItemBase>
