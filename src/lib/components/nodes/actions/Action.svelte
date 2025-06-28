<script lang="ts">
	import { writeText } from '@tauri-apps/plugin-clipboard-manager';
	import { openUrl } from '@tauri-apps/plugin-opener';
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import { useActionRole } from '$lib/actions.svelte';
	import BaseAction from './BaseAction.svelte';
	import type { ActionCopyToClipboardProps, ActionOpenInBrowserProps } from '$lib/props';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: unknown[]) => void;
		displayAs?: 'item' | 'button';
	};

	let { nodeId, uiTree, onDispatch, displayAs = 'item' }: Props = $props();

	const { node, props: componentProps } = $derived.by(
		useTypedNode(() => ({
			nodeId,
			uiTree,
			type: ['Action', 'Action.Push', 'Action.CopyToClipboard', 'Action.OpenInBrowser']
		}))
	);
	const { isPrimaryAction, isSecondaryAction } = $derived.by(useActionRole(nodeId));

	const title = $derived.by(() => {
		if (!node || !componentProps) return '';

		switch (node.type) {
			case 'Action.CopyToClipboard':
				return componentProps.title ?? 'Copy to Clipboard';
			case 'Action.OpenInBrowser':
				return componentProps.title ?? 'Open in Browser';
			default:
				return componentProps.title ?? '';
		}
	});

	function handleClick() {
		if (!node || !componentProps) return;

		switch (node.type) {
			case 'Action.CopyToClipboard':
				const copyProps = componentProps as ActionCopyToClipboardProps;
				writeText(copyProps.content);
				onDispatch(nodeId, 'onCopy', []);
				break;

			case 'Action.OpenInBrowser':
				const openProps = componentProps as ActionOpenInBrowserProps;
				openUrl(openProps.url);
				onDispatch(nodeId, 'onOpenInBrowser', []);
				break;

			case 'Action.Push':
			case 'Action':
			default:
				onDispatch(nodeId, 'onAction', []);
				break;
		}
	}
</script>

{#if componentProps}
	<BaseAction
		{title}
		icon={componentProps.icon}
		shortcut={componentProps.shortcut}
		{isPrimaryAction}
		{isSecondaryAction}
		{displayAs}
		onclick={handleClick}
	/>
{/if}
