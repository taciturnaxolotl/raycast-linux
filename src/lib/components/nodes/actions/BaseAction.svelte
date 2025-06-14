<script lang="ts">
	import type { KeyboardShortcut } from '$lib/props/actions';
	import { DropdownMenuItem, DropdownMenuShortcut } from '$lib/components/ui/dropdown-menu';
	import { Button } from '$lib/components/ui/button';
	import { shortcutToText } from '$lib/renderKey';

	type Props = {
		title: string;
		shortcut?: KeyboardShortcut | null;
		isPrimaryAction?: boolean;
		displayAs?: 'item' | 'button';
		onclick: (event: MouseEvent) => void;
	};

	let {
		title,
		shortcut = undefined,
		isPrimaryAction = false,
		displayAs = 'item',
		onclick
	}: Props = $props();
</script>

{#if displayAs === 'button'}
	<Button {onclick} variant="ghost" size="sm">{title}</Button>
{:else}
	<DropdownMenuItem class="rounded-md p-2 text-left" {onclick}>
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
