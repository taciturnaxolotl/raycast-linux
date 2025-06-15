<script lang="ts">
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import Icon from '$lib/components/Icon.svelte';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
	};

	let { nodeId, uiTree, onDispatch }: Props = $props();

	const { props: itemProps } = $derived.by(
		useTypedNode(() => ({ nodeId, uiTree, type: 'List.Dropdown.Item' }))
	);
</script>

{#if itemProps}
	<DropdownMenu.RadioItem value={itemProps.value}>
		{#if itemProps.icon}
			<div class="mr-2 flex size-4 shrink-0 items-center justify-center">
				<Icon icon={itemProps.icon} class="size-4" />
			</div>
		{/if}
		{itemProps.title}
	</DropdownMenu.RadioItem>
{/if}
