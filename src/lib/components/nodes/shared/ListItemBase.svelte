<script lang="ts">
	import Icon from '$lib/components/Icon.svelte';
	import type { ImageLike } from '$lib/props';
	import type { HTMLButtonAttributes } from 'svelte/elements';
	import type { Snippet } from 'svelte';

	type Props = {
		title: string;
		subtitle?: string | null;
		icon?: ImageLike | null;
		isSelected: boolean;
		accessories?: Snippet;
	} & HTMLButtonAttributes;

	let { title, subtitle, icon, isSelected, accessories, ...restProps }: Props = $props();
</script>

<button
	type="button"
	class="hover:bg-accent/50 flex w-full items-center gap-3 px-4 py-2 text-left"
	class:bg-accent={isSelected}
	{...restProps}
>
	<div class="flex size-5 shrink-0 items-center justify-center">
		{#if icon}
			<Icon {icon} class="size-4" />
		{:else}
			<div class="size-4"></div>
		{/if}
	</div>

	<div class="flex flex-grow items-baseline gap-4 overflow-hidden">
		<p class="font-medium whitespace-nowrap">{title}</p>
		{#if subtitle}
			<p class="text-muted-foreground truncate">{subtitle}</p>
		{/if}
	</div>

	{#if accessories}
		<div class="ml-auto flex shrink-0 items-center gap-4">
			{@render accessories()}
		</div>
	{/if}
</button>
