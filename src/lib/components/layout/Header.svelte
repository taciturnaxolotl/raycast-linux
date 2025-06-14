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
	const navigationTitle = $derived(rootNode?.props.navigationTitle as string | undefined);
	const searchBarAccessoryId = $derived(rootNode?.namedChildren?.searchBarAccessory);
</script>

<header class="flex h-12 shrink-0 items-center border-b px-2">
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
			{#if searchBarAccessoryId}
				<div>
					<NodeRenderer nodeId={searchBarAccessoryId} {uiTree} {onDispatch} />
				</div>
			{/if}
		{:else if navigationTitle}
			<div class="flex h-full w-full items-center px-2 text-base font-medium">
				{navigationTitle}
			</div>
		{/if}
	</div>
</header>
