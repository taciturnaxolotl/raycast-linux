<script lang="ts">
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import { getTypedProps } from '$lib/props';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
	};

	let { nodeId, uiTree, onDispatch }: Props = $props();
	const { props: componentProps } = $derived.by(
		useTypedNode(() => ({ nodeId, uiTree, type: 'List.Item.Detail.Metadata.Label' }))
	);

	const textValue = $derived(
		typeof componentProps?.text === 'object' ? componentProps.text.value : componentProps?.text
	);
	const textColor = $derived(
		typeof componentProps?.text === 'object' ? componentProps.text.color : undefined
	);
</script>

{#if componentProps}
	<div>
		<h3 class="mb-1 text-xs font-medium text-gray-500 uppercase">{componentProps.title}</h3>
		<div class="flex items-center gap-2">
			{#if componentProps.icon}
				<Icon icon={componentProps.icon} class="size-4" />
			{/if}
			{#if textValue}
				<span class="text-sm" style:color={textColor}>{textValue}</span>
			{/if}
		</div>
	</div>
{/if}
