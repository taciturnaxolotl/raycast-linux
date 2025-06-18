<script lang="ts">
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import * as Command from '$lib/components/ui/command';
	import Icon from '$lib/components/Icon.svelte';
	import { Check } from '@lucide/svelte';
	import { cn } from '$lib/utils';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
		selectedValue?: string;
	};

	let { nodeId, uiTree, onDispatch, selectedValue }: Props = $props();

	const { props: itemProps } = $derived.by(
		useTypedNode(() => ({ nodeId, uiTree, type: 'Grid.Dropdown.Item' }))
	);
</script>

{#if itemProps}
	<Command.Item
		value={itemProps.title}
		onSelect={() => {
			onDispatch(nodeId, 'onSelect', [itemProps.value]);
		}}
	>
		<div class="flex gap-2">
			{#if itemProps.icon}
				<div class="flex size-4 shrink-0 items-center justify-center">
					<Icon icon={itemProps.icon} />
				</div>
			{/if}
			<span>{itemProps.title}</span>
		</div>
		<Check class={cn('ml-auto h-4 w-4', selectedValue !== itemProps.value && 'text-transparent')} />
	</Command.Item>
{/if}
