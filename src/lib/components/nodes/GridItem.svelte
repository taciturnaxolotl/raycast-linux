<script lang="ts">
	import type { GridItemProps } from '$lib/props';
	import type { HTMLButtonAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils';
	import type { GridInset } from '$lib/props/grid';

	type Props = {
		props: GridItemProps;
		selected: boolean;
		inset?: GridInset;
	} & HTMLButtonAttributes;

	let { props, selected, inset, ...restProps }: Props = $props();

	const paddingClass = $derived(() => {
		switch (inset) {
			case 'small':
				return 'p-1';
			case 'medium':
				return 'p-2';
			case 'large':
				return 'p-4';
			default:
				return 'px-4 py-2';
		}
	});
</script>

<button
	type="button"
	class={cn(
		'hover:bg-accent/50 flex w-full flex-col items-center gap-3 text-left',
		paddingClass,
		selected && 'bg-accent'
	)}
	{...restProps}
>
	<img src={props.content} alt={props.title} />

	{#if props.title}
		<span class="text-sm font-medium">{props.title}</span>
	{/if}
	{#if props.subtitle}
		<span class="text-muted-foreground text-xs">{props.subtitle}</span>
	{/if}
</button>
