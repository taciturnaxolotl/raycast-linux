<script lang="ts">
	import type { KeyboardShortcut } from '$lib/props/actions';
	import { DropdownMenuItem, DropdownMenuShortcut } from '$lib/components/ui/dropdown-menu';
	import { Button } from '$lib/components/ui/button';
	import { shortcutToText } from '$lib/renderKey';
	import type { ImageLike } from '$lib/props';
	import Icon from '$lib/components/Icon.svelte';

	type Props = {
		title: string;
		shortcut?: KeyboardShortcut | null;
		icon?: ImageLike;
		isPrimaryAction?: boolean;
		displayAs?: 'item' | 'button';
		onclick: (event: MouseEvent) => void;
	};

	let {
		title,
		shortcut = undefined,
		icon,
		isPrimaryAction = false,
		displayAs = 'item',
		onclick
	}: Props = $props();
</script>

{#if displayAs === 'button'}
	<Button {onclick} variant="ghost" size="sm">{title}</Button>
{:else}
	<DropdownMenuItem class="rounded-md p-2 text-left" {onclick}>
		{#if icon}
			<Icon {icon} class="size-4" />
		{/if}
		{title}
		{#if isPrimaryAction}
			<DropdownMenuShortcut>
				{shortcutToText({ key: 'enter', modifiers: [] })}
			</DropdownMenuShortcut>
		{:else if shortcut}
			<DropdownMenuShortcut>{shortcutToText(shortcut)}</DropdownMenuShortcut>
		{/if}
	</DropdownMenuItem>
{/if}
