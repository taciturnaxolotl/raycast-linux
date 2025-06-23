<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { UINode } from '$lib/types';

	type Props = {
		header: Snippet;
		content: Snippet;
		footer: Snippet;
		primaryAction?: UINode;
		secondaryAction?: UINode;
		onDispatch: (instanceId: number, handlerName: string, args: unknown[]) => void;
	};
	let { header, content, footer, primaryAction, secondaryAction, onDispatch }: Props = $props();

	function getActionHandlerName(type: string): string {
		switch (type) {
			case 'Action.CopyToClipboard':
				return 'onCopy';
			case 'Action.OpenInBrowser':
				return 'onOpenInBrowser';
			default:
				return 'onAction';
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			if (
				event.target instanceof HTMLElement &&
				event.target.closest('[data-slot="dropdown-menu-content"]')
			) {
				return;
			}
			if (event.ctrlKey && !event.metaKey && !event.shiftKey) {
				if (secondaryAction) {
					event.preventDefault();
					const handlerName = getActionHandlerName(secondaryAction.type);
					onDispatch(secondaryAction.id, handlerName, []);
				}
			} else if (!event.metaKey && !event.ctrlKey && !event.shiftKey) {
				if (primaryAction) {
					event.preventDefault();
					const handlerName = getActionHandlerName(primaryAction.type);
					onDispatch(primaryAction.id, handlerName, []);
				}
			}
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<main class="bg-background text-foreground flex h-screen flex-col">
	{@render header()}
	{@render content()}
	{@render footer()}
</main>
