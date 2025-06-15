<script lang="ts">
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import NodeRenderer from '$lib/components/NodeRenderer.svelte';
	import { ChevronsUpDown } from '@lucide/svelte';
	import { getTypedProps, type ListDropdownItemProps } from '$lib/props';
	import Icon from '$lib/components/Icon.svelte';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
	};

	let { nodeId, uiTree, onDispatch }: Props = $props();

	const { node, props: dropdownProps } = $derived.by(
		useTypedNode(() => ({ nodeId, uiTree, type: 'List.Dropdown' }))
	);

	const dropdownItems = $derived.by(() => {
		const items: ListDropdownItemProps[] = [];
		if (!node) return items;

		function traverse(children: number[]) {
			for (const childId of children) {
				const childNode = uiTree.get(childId);
				if (!childNode) continue;

				if (childNode.type === 'List.Dropdown.Item') {
					const itemProps = getTypedProps({
						id: childNode.id,
						children: [],
						props: childNode.props,
						type: 'List.Dropdown.Item'
					});
					if (itemProps?.value) {
						items.push(itemProps);
					}
				} else if (childNode.type === 'List.Dropdown.Section') {
					if (childNode.children) traverse(childNode.children);
				}
			}
		}

		if (node.children) traverse(node.children);
		return items;
	});

	const itemsMap = $derived.by(() => new Map(dropdownItems.map((i) => [i.value, i])));
	const firstItemValue = $derived.by(() => dropdownItems[0]?.value);

	let value = $state(dropdownProps?.value ?? dropdownProps?.defaultValue);
	let mounted = $state(false);

	$effect(() => {
		if (dropdownProps?.value !== undefined && dropdownProps.value !== value) {
			value = dropdownProps.value;
		}
	});

	$effect(() => {
		if (!mounted) {
			if (value === undefined && firstItemValue !== undefined) {
				value = firstItemValue;
				onDispatch(nodeId, 'onChange', [value]);
			}
			mounted = true;
		} else {
			if (value !== dropdownProps?.value) {
				if (value !== undefined) {
					onDispatch(nodeId, 'onChange', [value]);
				}
			}
		}
	});

	const selectedItem = $derived(itemsMap.get(value ?? ''));
</script>

{#if node && dropdownProps}
	<DropdownMenu.Root>
		<DropdownMenu.Trigger
			class="hover:bg-accent flex w-60 items-center gap-1 rounded-md border bg-transparent px-2 py-1 text-sm focus:outline-none"
			title={dropdownProps.tooltip}
		>
			{#if selectedItem?.icon}
				<div class="mr-2 flex size-4 shrink-0 items-center justify-center">
					<Icon icon={selectedItem.icon} class="size-4" />
				</div>
			{/if}
			<span class="truncate"
				>{selectedItem?.title ?? dropdownProps?.placeholder ?? 'Select...'}</span
			>
			<ChevronsUpDown class="ml-auto h-4 w-4 shrink-0 opacity-50" />
		</DropdownMenu.Trigger>
		<DropdownMenu.Content class="w-60">
			<DropdownMenu.RadioGroup bind:value>
				{#each node.children as childId (childId)}
					<NodeRenderer nodeId={childId} {uiTree} {onDispatch} />
				{/each}
			</DropdownMenu.RadioGroup>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
{/if}
