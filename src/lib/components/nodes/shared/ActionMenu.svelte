<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Button } from '$lib/components/ui/button';
	import { Kbd } from '$lib/components/ui/kbd';
	import type { Snippet } from 'svelte';
	import { setContext } from 'svelte';
	import KeyboardShortcut from '$lib/components/KeyboardShortcut.svelte';

	type Props = {
		children: Snippet;
		primaryActionNodeId?: number;
		secondaryActionNodeId?: number;
	};

	let { children, primaryActionNodeId, secondaryActionNodeId }: Props = $props();
	let open = $state(false);

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'k' && (e.ctrlKey || e.metaKey)) {
			e.preventDefault();
			open = !open;
		}
	}

	setContext('ActionPanelContext', {
		primaryActionNodeId: () => primaryActionNodeId,
		secondaryActionNodeId: () => secondaryActionNodeId
	});
</script>

<svelte:window onkeydown={handleKeydown} />

<DropdownMenu.Root bind:open>
	<DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button {...props} variant="ghost" size="sm">
				Actions
				<KeyboardShortcut shortcut={{ key: 'k', modifiers: ['cmd'] }} />
			</Button>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content class="w-80">
		{@render children()}
	</DropdownMenu.Content>
</DropdownMenu.Root>
