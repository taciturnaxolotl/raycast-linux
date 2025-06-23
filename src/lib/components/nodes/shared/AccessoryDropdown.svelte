<script lang="ts">
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import { tick, setContext } from 'svelte';
	import { ChevronsUpDown } from '@lucide/svelte';
	import * as Command from '$lib/components/ui/command';
	import * as Popover from '$lib/components/ui/popover';
	import { Button } from '$lib/components/ui/button';
	import NodeRenderer from '$lib/components/NodeRenderer.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import { getDropdownItems } from '$lib/components/nodes/shared/dropdown';
	import type { ListDropdownItemProps } from '$lib/props';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: unknown[]) => void;
	};

	let { nodeId, uiTree, onDispatch }: Props = $props();

	const { node, props: componentProps } = $derived.by(
		useTypedNode(() => ({
			nodeId,
			uiTree,
			type: ['List.Dropdown', 'Grid.Dropdown']
		}))
	);

	const isControlled = $derived(componentProps?.value !== undefined);
	const dropdownItems = $derived(node ? getDropdownItems(node, uiTree) : []);
	const itemsMap = $derived(new Map(dropdownItems.map((i: ListDropdownItemProps) => [i.value, i])));
	const firstItemValue = $derived(dropdownItems[0]?.value);

	let internalValue = $state<string | undefined>();
	let isInitialized = $state(false);
	let open = $state(false);
	let triggerRef = $state<HTMLButtonElement | null>(null);

	const displayValue = $derived(isControlled ? componentProps?.value : internalValue);
	const selectedItem = $derived(itemsMap.get(displayValue ?? ''));

	$effect(() => {
		if (componentProps && !isInitialized) {
			const initial = componentProps.defaultValue;
			if (initial !== undefined) {
				internalValue = initial;
			} else if (firstItemValue !== undefined) {
				onDispatch(nodeId, 'onChange', [firstItemValue]);
				if (!isControlled) {
					internalValue = firstItemValue;
				}
			}
			isInitialized = true;
		}
	});

	$effect(() => {
		if (isControlled && componentProps) {
			internalValue = componentProps.value;
		}
	});

	$effect(() => {
		if (isInitialized && !isControlled && internalValue !== undefined) {
			onDispatch(nodeId, 'onChange', [internalValue]);
		}
	});

	function onSelect(value: string) {
		if (!isControlled) {
			internalValue = value;
		}
		onDispatch(nodeId, 'onChange', [value]);
		open = false;
		tick().then(() => {
			triggerRef?.focus();
		});
	}

	setContext('unified-dropdown', {
		displayValue: () => displayValue,
		onSelect
	});
</script>

{#if node && componentProps}
	<Popover.Root bind:open>
		<Popover.Trigger bind:ref={triggerRef}>
			{#snippet child({ props: popoverTriggerProps })}
				<Button
					{...popoverTriggerProps}
					variant="outline"
					class="w-60 justify-between"
					role="combobox"
					aria-expanded={open}
					title={componentProps.tooltip}
				>
					<div class="flex items-center gap-2">
						{#if selectedItem?.icon}
							<div class="flex size-4 shrink-0 items-center justify-center">
								<Icon icon={selectedItem.icon} />
							</div>
						{/if}
						<span class="truncate">
							{selectedItem?.title ?? componentProps?.placeholder ?? 'Select...'}
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
					{#each node.children as childId (childId)}
						<NodeRenderer nodeId={childId} {uiTree} {onDispatch} selectedValue={displayValue} />
					{/each}
				</Command.List>
			</Command.Root>
		</Popover.Content>
	</Popover.Root>
{/if}
