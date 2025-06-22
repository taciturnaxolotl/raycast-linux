<script lang="ts" generics="T extends { id: string | number; itemType: string }">
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
		const selectedItemElement = listElement?.querySelector(`[data-index="${selectedIndex}"]`);
		selectedItemElement?.scrollIntoView({ block: 'nearest' });
	});

	$effect(() => {
		if (
			selectedIndex < 0 ||
			selectedIndex >= items.length ||
			items[selectedIndex]?.itemType !== 'item'
		) {
			let iterations = 0;
			do {
				selectedIndex = (selectedIndex + 1) % items.length;
				iterations++;
				if (iterations > items.length) {
					selectedIndex = 0;
					break;
				}
			} while (items[selectedIndex]?.itemType !== 'item');
		}
	});

	function handleKeydown(event: KeyboardEvent) {
		if (items.length === 0) return;

		const itemIndices = items
			.map((item, i) => (item.itemType === 'item' ? i : -1))
			.filter((i) => i !== -1);
		if (itemIndices.length === 0) return;

		const currentItemIndexInSublist = itemIndices.indexOf(selectedIndex);
		let newIndexInSublist = currentItemIndexInSublist;

		if (event.key === 'ArrowDown') {
			event.preventDefault();
			if (newIndexInSublist === -1) {
				newIndexInSublist = 0;
			} else {
				newIndexInSublist = Math.min(itemIndices.length - 1, newIndexInSublist + 1);
			}
			selectedIndex = itemIndices[newIndexInSublist];
		} else if (event.key === 'ArrowUp') {
			event.preventDefault();
			if (newIndexInSublist === -1) {
				newIndexInSublist = itemIndices.length - 1;
			} else {
				newIndexInSublist = Math.max(0, newIndexInSublist - 1);
			}
			selectedIndex = itemIndices[newIndexInSublist];
		} else if (event.key === 'Enter') {
			const selectedItem = items[selectedIndex];
			if (selectedItem?.itemType === 'item') {
				event.preventDefault();
				onenter(selectedItem);
			}
		}
	}

	onMount(() => {
		if (autofocus && listElement) {
			listElement.focus();
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
					if (item.itemType === 'item') {
						selectedIndex = index;
						onenter(item);
					}
				}
			})}
		</div>
	{/each}
</div>
