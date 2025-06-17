<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { ArrowLeft } from '@lucide/svelte';
	import type { UINode } from '$lib/types';
	import NodeRenderer from '../NodeRenderer.svelte';

	type Props = {
		rootNode: UINode | undefined;
		searchText: string;
		onPopView: () => void;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
		uiTree: Map<number, UINode>;
		showBackButton: boolean;
	};

	let {
		rootNode,
		searchText = $bindable(),
		onPopView,
		onDispatch,
		uiTree,
		showBackButton
	}: Props = $props();

	const viewType = $derived(rootNode?.type);
	const placeholder = $derived((rootNode?.props.searchBarPlaceholder as string) ?? 'Search...');
	const searchBarAccessoryId = $derived(rootNode?.namedChildren?.searchBarAccessory);
	const isLoading = $derived(rootNode?.props.isLoading as boolean | undefined);
</script>

<header class="relative flex h-12 shrink-0 items-center px-2">
	{#if showBackButton}
		<Button variant="ghost" size="icon" onclick={onPopView}>
			<ArrowLeft class="size-5" />
		</Button>
	{/if}

	<div class="flex flex-grow items-center">
		{#if viewType === 'List' || viewType === 'Grid'}
			<Input
				class="rounded-none border-none !bg-transparent pr-0"
				{placeholder}
				bind:value={searchText}
				autofocus
			/>
			{#key searchBarAccessoryId}
				{#if searchBarAccessoryId}
					<div>
						<NodeRenderer nodeId={searchBarAccessoryId} {uiTree} {onDispatch} />
					</div>
				{/if}
			{/key}
		{/if}
	</div>

	<div class="bg-muted absolute right-0 bottom-0 left-0 h-[2px]">
		{#if isLoading}
			<div class="raycast-loader h-full w-full"></div>
		{/if}
	</div>
</header>
