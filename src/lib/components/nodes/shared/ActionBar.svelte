<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { ImageLike } from '$lib/props';
	import Icon from '$lib/components/Icon.svelte';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import type { ButtonProps } from '$lib/components/ui/button';

	type Props = {
		title?: string;
		icon?: ImageLike | null;
		primaryAction?: Snippet<[{ props: ButtonProps }]>;
		actions?: Snippet;
	};

	let { title, icon, primaryAction, actions }: Props = $props();
</script>

<footer class="bg-card flex h-12 shrink-0 items-center border-t px-4">
	{#if title || icon}
		<div class="flex min-w-0 items-center gap-2">
			{#if icon}
				<Icon {icon} class="size-5 shrink-0" />
			{/if}
			{#if title}
				<span class="text-muted-foreground truncate text-sm">{title}</span>
			{/if}
		</div>
	{/if}

	<div class="ml-auto flex items-center gap-2">
		{#if primaryAction}
			{@render primaryAction({ props: { variant: 'ghost', size: 'sm' } })}
		{/if}

		{#if actions}
			{#if primaryAction}
				<div class="group flex items-center">
					<Separator
						orientation="vertical"
						class="!h-4 !w-px transition-opacity group-hover:opacity-0"
					/>
				</div>
			{/if}
			{@render actions()}
		{/if}
	</div>
</footer>
