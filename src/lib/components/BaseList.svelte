<script lang="ts" generics="T extends { id: string | number }">
	import { onMount } from 'svelte';
	import type { Snippet } from 'svelte';

	type Props = {
		items: T[];
		itemSnippet: Snippet<[{ item: T; isSelected: boolean; onclick: () => void }]>;
		autofocus?: boolean;
		onenter: (item: T) => void;
		selectedIndex?: number;
		listElement?: HTMLElement | null;
	};

	let {
		items,
		itemSnippet,
		autofocus = false,
		onenter,
		selectedIndex = $bindable(0),
		listElement = $bindable()
	}: Props = $props();

	$effect(() => {
		if (selectedIndex >= items.length) {
			selectedIndex = Math.max(0, items.length - 1);
		}
	});

	$effect(() => {
		const selectedItemElement = listElement?.querySelector(`[data-index="${selectedIndex}"]`);
		selectedItemElement?.scrollIntoView({ block: 'nearest' });
	});

	function handleKeydown(event: KeyboardEvent) {
		if (items.length === 0) return;

		if (event.key === 'ArrowDown') {
			event.preventDefault();
			selectedIndex = (selectedIndex + 1) % items.length;
		} else if (event.key === 'ArrowUp') {
			event.preventDefault();
			selectedIndex = (selectedIndex - 1 + items.length) % items.length;
		} else if (event.key === 'Enter') {
			event.preventDefault();
			const selectedItem = items[selectedIndex];
			if (selectedItem) {
				onenter(selectedItem);
			}
		}
	}

	onMount(() => {
		if (autofocus) {
			listElement?.focus();
		}
	});
</script>

<svelte:window onkeydown={handleKeydown} />

<div bind:this={listElement} class="contents" tabindex={-1}>
	{#each items as item, index (item.id)}
		<div data-index={index}>
			{@render itemSnippet({
				item,
				isSelected: selectedIndex === index,
				onclick: () => {
					selectedIndex = index;
					onenter(item);
				}
			})}
		</div>
	{/each}
</div>
