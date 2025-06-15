<script lang="ts">
	import { writeText } from '@tauri-apps/plugin-clipboard-manager';
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import { getContext } from 'svelte';
	import BaseAction from './BaseAction.svelte';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
		displayAs?: 'item' | 'button';
	};

	let { nodeId, uiTree, onDispatch, displayAs = 'item' }: Props = $props();
	const { props: componentProps } = $derived.by(
		useTypedNode(() => ({ nodeId, uiTree, type: 'Action.CopyToClipboard' }))
	);

	const context: { primaryActionNodeId?: number; secondaryActionNodeId?: number } | undefined =
		getContext('ActionPanelContext');
	const isPrimaryAction = $derived(context?.primaryActionNodeId === nodeId);
	const isSecondaryAction = $derived(context?.secondaryActionNodeId === nodeId);

	function handleClick() {
		if (componentProps) {
			writeText(componentProps.content);
			onDispatch(nodeId, 'onCopy', []);
		}
	}
</script>

{#if componentProps}
	<BaseAction
		title={componentProps.title ?? 'Copy to Clipboard'}
		shortcut={componentProps.shortcut}
		icon={componentProps.icon}
		{isPrimaryAction}
		{isSecondaryAction}
		{displayAs}
		onclick={handleClick}
	/>
{/if}
