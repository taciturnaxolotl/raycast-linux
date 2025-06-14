<script lang="ts">
	import { writeText } from '@tauri-apps/plugin-clipboard-manager';
	import type { UINode } from '$lib/types';
	import { DropdownMenuItem, DropdownMenuShortcut } from '$lib/components/ui/dropdown-menu';
	import { useTypedNode } from '$lib/node.svelte';
	import { getContext } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { shortcutToText } from '$lib/renderKey';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
		displayAs?: 'item' | 'button';
	};

	let { nodeId, uiTree, onDispatch, displayAs = 'item' }: Props = $props();
	const { node, props: componentProps } = $derived.by(
		useTypedNode(() => ({ nodeId, uiTree, type: 'Action.CopyToClipboard' }))
	);

	const context: { primaryActionNodeId?: number } | undefined = getContext('ActionPanelContext');
	const isPrimaryAction = $derived(context?.primaryActionNodeId === nodeId);

	function handleClick() {
		if (componentProps) {
			writeText(componentProps.content);
			onDispatch(nodeId, 'onCopy', []);
		}
	}
</script>

{#if node && componentProps}
	{#if displayAs === 'button'}
		<Button onclick={handleClick} variant="ghost" size="sm">
			{componentProps.title ?? 'Copy to Clipboard'}
		</Button>
	{:else}
		<DropdownMenuItem class="rounded-md p-2 text-left" onclick={handleClick}>
			{componentProps.title ?? 'Copy to Clipboard'}
			{#if isPrimaryAction}
				<DropdownMenuShortcut>
					{shortcutToText({ key: 'enter', modifiers: [] })}
				</DropdownMenuShortcut>
			{:else if componentProps.shortcut}
				<DropdownMenuShortcut>{shortcutToText(componentProps.shortcut)}</DropdownMenuShortcut>
			{/if}
		</DropdownMenuItem>
	{/if}
{/if}
