<script lang="ts">
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import { tick, setContext } from 'svelte';
	import { ChevronsUpDownIcon, ChevronsUpDown } from '@lucide/svelte';
	import * as Command from '$lib/components/ui/command';
	import * as Popover from '$lib/components/ui/popover';
	import { Button } from '$lib/components/ui/button';
	import NodeRenderer from '$lib/components/NodeRenderer.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import type {
		ListDropdownItemProps,
		FormDropdownProps,
		GridDropdownProps,
		ListDropdownProps
	} from '$lib/props';
	import { getTypedProps } from '$lib/props';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
	};

	let { nodeId, uiTree, onDispatch }: Props = $props();

	const { node, props: componentProps } = $derived.by(
		useTypedNode(() => ({
			nodeId,
			uiTree,
			type: ['List.Dropdown', 'Grid.Dropdown', 'Form.Dropdown']
		}))
	);

	const isFormControl = $derived(node?.type === 'Form.Dropdown');
	const formProps = $derived(isFormControl ? (componentProps as FormDropdownProps | null) : null);
	const listGridProps = $derived(
		!isFormControl ? (componentProps as (ListDropdownProps | GridDropdownProps) | null) : null
	);

	const isControlled = $derived(componentProps?.value !== undefined);

	const dropdownItems = $derived.by(() => {
		const items: ListDropdownItemProps[] = [];
		if (!node) return items;

		function traverse(children: number[]) {
			for (const childId of children) {
				const childNode = uiTree.get(childId);
				if (!childNode) continue;

				if (
					childNode.type === 'List.Dropdown.Item' ||
					childNode.type === 'Grid.Dropdown.Item' ||
					childNode.type === 'Form.Dropdown.Item'
				) {
					const itemProps = getTypedProps({
						...childNode,
						type: 'List.Dropdown.Item'
					});
					if (itemProps) {
						items.push(itemProps);
					}
				} else if (
					childNode.type === 'List.Dropdown.Section' ||
					childNode.type === 'Grid.Dropdown.Section'
				) {
					if (childNode.children) traverse(childNode.children);
				}
			}
		}

		if (node.children) traverse(node.children);
		return items;
	});

	const itemsMap = $derived.by(() => new Map(dropdownItems.map((i) => [i.value, i])));
	const firstItemValue = $derived.by(() => dropdownItems[0]?.value);

	let internalValue = $state(componentProps?.defaultValue);
	let mounted = $state(false);
	let open = $state(false);
	let triggerRef = $state<HTMLButtonElement>(null!);

	const displayValue = $derived(isControlled ? componentProps?.value : internalValue);

	$effect(() => {
		if (componentProps?.value !== undefined && componentProps.value !== internalValue) {
			internalValue = componentProps.value;
		}
	});

	$effect(() => {
		if (!mounted) {
			if (internalValue === undefined && firstItemValue !== undefined) {
				onDispatch(nodeId, 'onChange', [firstItemValue]);
				if (!isControlled) {
					internalValue = firstItemValue;
				}
			}
			mounted = true;
		} else {
			if (internalValue !== componentProps?.value) {
				if (internalValue !== undefined) {
					onDispatch(nodeId, 'onChange', [internalValue]);
				}
			}
		}
	});

	const selectedItem = $derived(itemsMap.get(displayValue ?? ''));

	function closeAndFocusTrigger() {
		open = false;
		tick().then(() => {
			triggerRef?.focus();
		});
	}

	function onSelect(value: string) {
		if (!isControlled) {
			internalValue = value;
		}
		onDispatch(nodeId, 'onChange', [value]);
		closeAndFocusTrigger();
	}

	setContext('unified-dropdown', {
		displayValue: () => displayValue,
		onSelect
	});
</script>

{#if node && componentProps}
	{#if isFormControl && formProps}
		<div class="flex gap-4">
			<label for={formProps.id} class="text-muted-foreground pt-2 text-right text-sm font-medium">
				{formProps.title}
			</label>
			<div class="w-full">
				<Popover.Root bind:open>
					<Popover.Trigger bind:ref={triggerRef}>
						{#snippet child({ props: popoverTriggerProps })}
							<Button
								{...popoverTriggerProps}
								variant="outline"
								class="w-full justify-between"
								role="combobox"
								aria-expanded={open}
							>
								{selectedItem?.title || formProps.placeholder || 'Select option...'}
								<ChevronsUpDownIcon class="opacity-50" />
							</Button>
						{/snippet}
					</Popover.Trigger>
					<Popover.Content class="w-full p-0">
						<Command.Root>
							{#if formProps.filtering !== false}
								<Command.Input placeholder="Search..." />
							{/if}
							<Command.List>
								<Command.Empty>No option found.</Command.Empty>
								{#each node.children as childId (childId)}
									<NodeRenderer nodeId={childId} {uiTree} {onDispatch} />
								{/each}
							</Command.List>
						</Command.Root>
					</Popover.Content>
				</Popover.Root>
				{#if formProps.error}
					<p class="mt-1 text-xs text-red-600">{formProps.error}</p>
				{/if}
				{#if formProps.info}
					<p class="mt-1 text-xs text-gray-500">{formProps.info}</p>
				{/if}
			</div>
		</div>
	{:else if listGridProps}
		<Popover.Root bind:open>
			<Popover.Trigger bind:ref={triggerRef}>
				{#snippet child({ props: popoverTriggerProps })}
					<Button
						{...popoverTriggerProps}
						variant="outline"
						class="w-60 justify-between"
						role="combobox"
						aria-expanded={open}
						title={listGridProps.tooltip}
					>
						<div class="flex items-center gap-2">
							{#if selectedItem?.icon}
								<div class="flex size-4 shrink-0 items-center justify-center">
									<Icon icon={selectedItem.icon} />
								</div>
							{/if}
							<span class="truncate">
								{selectedItem?.title ?? listGridProps?.placeholder ?? 'Select...'}
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
{/if}
