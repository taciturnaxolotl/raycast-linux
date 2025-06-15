<script lang="ts">
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import * as Command from '$lib/components/ui/command';
	import * as Popover from '$lib/components/ui/popover';
	import { Button } from '$lib/components/ui/button';
	import NodeRenderer from '$lib/components/NodeRenderer.svelte';
	import { ChevronsUpDown } from '@lucide/svelte';
	import { getTypedProps, type GridDropdownItemProps } from '$lib/props';
	import Icon from '$lib/components/Icon.svelte';
	import { tick } from 'svelte';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
	};

	let { nodeId, uiTree, onDispatch }: Props = $props();

	const { node, props: dropdownProps } = $derived.by(
		useTypedNode(() => ({ nodeId, uiTree, type: 'Grid.Dropdown' }))
	);

	const dropdownItems = $derived.by(() => {
		const items: GridDropdownItemProps[] = [];
		if (!node) return items;

		function traverse(children: number[]) {
			for (const childId of children) {
				const childNode = uiTree.get(childId);
				if (!childNode) continue;

				if (childNode.type === 'Grid.Dropdown.Item') {
					const itemProps = getTypedProps({
						id: 1,
						children: [],
						props: childNode.props,
						type: 'Grid.Dropdown.Item'
					});
					if (itemProps) {
						items.push(itemProps);
					}
				} else if (childNode.type === 'Grid.Dropdown.Section') {
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
	let open = $state(false);
	let triggerRef = $state<HTMLButtonElement>(null!);

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

	function closeAndFocusTrigger() {
		open = false;
		tick().then(() => {
			triggerRef.focus();
		});
	}

	function handleItemSelect(childNodeId: number, handlerName: string, args: any[]) {
		if (handlerName === 'onSelect') {
			value = args[0];
			closeAndFocusTrigger();
		}
		onDispatch(childNodeId, handlerName, args);
	}
</script>

{#if node && dropdownProps}
	<Popover.Root bind:open>
		<Popover.Trigger bind:ref={triggerRef}>
			{#snippet child({ props })}
				<Button
					{...props}
					variant="outline"
					class="w-60 justify-between"
					role="combobox"
					aria-expanded={open}
					title={dropdownProps.tooltip}
				>
					<div class="flex items-center gap-2">
						{#if selectedItem?.icon}
							<div class="flex size-4 shrink-0 items-center justify-center">
								<Icon icon={selectedItem.icon} />
							</div>
						{/if}
						<span class="truncate">
							{selectedItem?.title ?? dropdownProps?.placeholder ?? 'Select...'}
						</span>
					</div>
					<ChevronsUpDown class="h-4 w-4 shrink-0 opacity-50" />
				</Button>
			{/snippet}
		</Popover.Trigger>
		<Popover.Content class="w-60 p-0">
			<Command.Root>
				<Command.Input placeholder="Search..." />
				<Command.List>
					<Command.Empty>No items found.</Command.Empty>
					<Command.Group>
						{#each node.children as childId (childId)}
							<NodeRenderer
								nodeId={childId}
								{uiTree}
								onDispatch={handleItemSelect}
								selectedValue={value}
							/>
						{/each}
					</Command.Group>
				</Command.List>
			</Command.Root>
		</Popover.Content>
	</Popover.Root>
{/if}
