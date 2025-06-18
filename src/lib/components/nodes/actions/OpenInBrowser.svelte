<script lang="ts">
	import { openUrl } from '@tauri-apps/plugin-opener';
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import { useActionRole } from '$lib/actions.svelte';
	import BaseAction from './BaseAction.svelte';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
		displayAs?: 'item' | 'button';
	};

	let { nodeId, uiTree, onDispatch, displayAs = 'item' }: Props = $props();
	const { props: componentProps } = $derived.by(
		useTypedNode(() => ({ nodeId, uiTree, type: 'Action.OpenInBrowser' }))
	);
	const { isPrimaryAction, isSecondaryAction } = $derived.by(useActionRole(nodeId));

	function handleClick() {
		if (componentProps) {
			openUrl(componentProps.url);
			onDispatch(nodeId, 'onOpenInBrowser', []);
		}
	}
</script>

{#if componentProps}
	<BaseAction
		title={componentProps.title ?? 'Open in Browser'}
		shortcut={componentProps.shortcut}
		icon={componentProps.icon}
		{isPrimaryAction}
		{isSecondaryAction}
		{displayAs}
		onclick={handleClick}
	/>
{/if}
