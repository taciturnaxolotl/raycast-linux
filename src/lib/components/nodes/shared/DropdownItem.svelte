<script lang="ts">
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import { getContext } from 'svelte';
	import CheckIcon from '@lucide/svelte/icons/check';
	import * as Command from '$lib/components/ui/command';
	import Icon from '$lib/components/Icon.svelte';
	import { cn } from '$lib/utils';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: unknown[]) => void;
		selectedValue?: string;
	};

	let { nodeId, uiTree, onDispatch, selectedValue }: Props = $props();

	const { props: componentProps } = $derived.by(
		useTypedNode(() => ({
			nodeId,
			uiTree,
			type: ['List.Dropdown.Item', 'Grid.Dropdown.Item', 'Form.Dropdown.Item']
		}))
	);

	const dropdownContext = getContext<{
		onSelect: (value: string) => void;
		displayValue: () => string | undefined;
	}>('unified-dropdown');

	const isSelected = $derived(
		selectedValue
			? selectedValue === componentProps?.value
			: dropdownContext?.displayValue() === componentProps?.value
	);
</script>

{#if componentProps && dropdownContext}
	<Command.Item
		value={componentProps.value ?? componentProps.title}
		keywords={[...(componentProps.keywords ?? []), componentProps.title]}
		onSelect={() => {
			dropdownContext.onSelect(componentProps.value);
			onDispatch(nodeId, 'onSelect', [componentProps.value]);
		}}
	>
		<CheckIcon class={cn('mr-2 size-4', !isSelected && 'text-transparent')} />
		{#if componentProps.icon}
			<Icon icon={componentProps.icon} class="mr-2 size-4" />
		{/if}
		{componentProps.title}
	</Command.Item>
{/if}
