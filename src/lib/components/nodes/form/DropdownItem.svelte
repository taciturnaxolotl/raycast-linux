<script lang="ts">
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import { getContext } from 'svelte';
	import CheckIcon from '@lucide/svelte/icons/check';
	import * as Command from '$lib/components/ui/command';
	import Icon from '$lib/components/Icon.svelte';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
	};

	let { nodeId, uiTree, onDispatch }: Props = $props();

	const { node, props: componentProps } = $derived.by(
		useTypedNode(() => ({ nodeId, uiTree, type: 'Form.Dropdown.Item' }))
	);

	const dropdownContext = getContext<{
		displayValue: () => string | undefined;
		onSelect: (value: string) => void;
		cn: (...args: any[]) => string;
	}>('form-dropdown');
</script>

{#if componentProps && dropdownContext}
	<Command.Item
		value={componentProps.value}
		keywords={[...(componentProps.keywords ?? []), componentProps.title]}
		onSelect={() => dropdownContext.onSelect(componentProps.value)}
	>
		<CheckIcon
			class={dropdownContext.cn(
				'mr-2 size-4',
				dropdownContext.displayValue() !== componentProps.value && 'text-transparent'
			)}
		/>

		{#if componentProps.icon}
			<Icon icon={componentProps.icon} class="mr-2 size-4" />
		{/if}

		{componentProps.title}
	</Command.Item>
{/if}
