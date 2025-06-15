<script lang="ts">
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import { tick, setContext } from 'svelte';
	import { ChevronsUpDownIcon } from '@lucide/svelte';
	import * as Command from '$lib/components/ui/command';
	import * as Popover from '$lib/components/ui/popover';
	import { Button } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import NodeRenderer from '$lib/components/NodeRenderer.svelte';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
	};

	let { nodeId, uiTree, onDispatch }: Props = $props();

	const { node, props: componentProps } = $derived.by(
		useTypedNode(() => ({ nodeId, uiTree, type: 'Form.Dropdown' }))
	);

	const isControlled = $derived(componentProps?.value !== undefined);

	let internalValue = $state('');
	let isInitialized = false;
	let open = $state(false);
	let triggerRef = $state<HTMLButtonElement>(null!);

	$effect(() => {
		if (componentProps && !isInitialized && !isControlled) {
			internalValue = componentProps.defaultValue ?? '';
			isInitialized = true;
		}
	});

	const displayValue = $derived(isControlled ? componentProps?.value : internalValue);

	const items = $derived(() => {
		if (!node?.children) return [];
		return node.children
			.map((childId) => {
				const childNode = uiTree.get(childId);
				if (childNode?.type === 'Form.Dropdown.Item') {
					return {
						id: childId,
						value: childNode.props.value as string,
						title: childNode.props.title as string,
						keywords: childNode.props.keywords as string[] | undefined,
						icon: childNode.props.icon as string | undefined
					};
				}
				return null;
			})
			.filter((item): item is NonNullable<typeof item> => item !== null);
	});

	const selectedItem = $derived(() => {
		return items().find((item) => item.value === displayValue);
	});

	function closeAndFocusTrigger() {
		open = false;
		tick().then(() => {
			triggerRef.focus();
		});
	}

	function onSelect(value: string) {
		const newValue = value;
		if (!isControlled) {
			internalValue = newValue;
		}
		onDispatch(nodeId, 'onChange', [newValue]);
		closeAndFocusTrigger();
	}

	setContext('form-dropdown', {
		displayValue: () => displayValue,
		onSelect,
		cn
	});
</script>

{#if componentProps}
	<div class="flex gap-4">
		<label
			for={componentProps.id}
			class="text-muted-foreground pt-2 text-right text-sm font-medium"
		>
			{componentProps.title}
		</label>
		<div class="w-full">
			<Popover.Root bind:open>
				<Popover.Trigger bind:ref={triggerRef}>
					{#snippet child({ props })}
						<Button
							{...props}
							variant="outline"
							class="w-full justify-between"
							role="combobox"
							aria-expanded={open}
						>
							{selectedItem()?.title || componentProps.placeholder || 'Select option...'}
							<ChevronsUpDownIcon class="opacity-50" />
						</Button>
					{/snippet}
				</Popover.Trigger>
				<Popover.Content class="w-full p-0">
					<Command.Root>
						{#if componentProps.filtering !== false}
							<Command.Input placeholder="Search..." />
						{/if}
						<Command.List>
							<Command.Empty>No option found.</Command.Empty>
							<Command.Group>
								{#if node}
									{#each node.children as childId (childId)}
										<NodeRenderer nodeId={childId} {uiTree} {onDispatch} />
									{/each}
								{/if}
							</Command.Group>
						</Command.List>
					</Command.Root>
				</Popover.Content>
			</Popover.Root>
			{#if componentProps.error}
				<p class="mt-1 text-xs text-red-600">{componentProps.error}</p>
			{/if}
			{#if componentProps.info}
				<p class="mt-1 text-xs text-gray-500">{componentProps.info}</p>
			{/if}
		</div>
	</div>
{/if}
