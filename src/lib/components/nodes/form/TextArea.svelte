<script lang="ts">
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import { Textarea } from '$lib/components/ui/textarea';

	type Props = {
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
	};

	let { nodeId, uiTree, onDispatch }: Props = $props();

	const { node, props: componentProps } = $derived.by(
		useTypedNode(() => ({ nodeId, uiTree, type: 'Form.TextArea' }))
	);

	let value = $state(componentProps?.defaultValue ?? '');

	$effect(() => {
		if (componentProps?.value !== undefined && componentProps.value !== value) {
			value = componentProps.value;
		}
	});
</script>

{#if node && componentProps}
	<div class="flex gap-4">
		<label
			for={componentProps.id}
			class="text-muted-foreground pt-2 text-right text-sm font-medium"
		>
			{componentProps.title}
		</label>
		<div class="w-full">
			<Textarea
				id={componentProps.id}
				placeholder={componentProps.placeholder}
				bind:value
				oninput={(e) => onDispatch(nodeId, 'onChange', [e.currentTarget.value])}
				onblur={(e) => onDispatch(nodeId, 'onBlur', [e])}
				aria-invalid={!!componentProps.error}
			/>
			{#if componentProps.error}
				<p class="mt-1 text-xs text-red-600">{componentProps.error}</p>
			{/if}
			{#if componentProps.info}
				<p class="mt-1 text-xs text-gray-500">{componentProps.info}</p>
			{/if}
		</div>
	</div>
{/if}
