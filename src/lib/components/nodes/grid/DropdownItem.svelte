<script lang="ts">
	import type { UINode } from '$lib/types';
	import { useTypedNode } from '$lib/node.svelte';
	import * as Command from '$lib/components/ui/command';
	import Icon from '$lib/components/Icon.svelte';
	import { RaycastIconSchema } from '$lib/props';
	import { convertFileSrc } from '@tauri-apps/api/core';
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

	const iconInfo = $derived.by(() => {
		const icon = itemProps?.icon;
		if (!icon) return null;

		const absolutePath = '/home/byte/code/raycast-linux/sidecar/dist/plugin/assets/';

		if (typeof icon === 'string') {
			if (RaycastIconSchema.safeParse(icon).success) {
				return { type: 'raycast' as const, name: icon };
			}
			return { type: 'image' as const, src: convertFileSrc(absolutePath + icon) };
		}

		if (typeof icon === 'object' && 'source' in icon) {
			return {
				type: 'image' as const,
				src: convertFileSrc(absolutePath + icon.source),
				mask: icon.mask
			};
		}
		return null;
	});

	const maskStyles = $derived(
		iconInfo?.type === 'image' && iconInfo.mask === 'Circle' ? 'border-radius: 50%;' : ''
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
			{#if iconInfo}
				<div class="flex size-4 shrink-0 items-center justify-center">
					{#if iconInfo.type === 'raycast'}
						<Icon icon={iconInfo.name} />
					{:else if iconInfo.type === 'image'}
						<img src={iconInfo.src} alt="" class="size-full object-cover" style={maskStyles} />
					{/if}
				</div>
			{/if}
			<span>{itemProps.title}</span>
		</div>
		<Check class={cn('ml-auto h-4 w-4', selectedValue !== itemProps.value && 'text-transparent')} />
	</Command.Item>
{/if}
