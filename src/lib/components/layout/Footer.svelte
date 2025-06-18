<script lang="ts">
	import type { UINode } from '$lib/types';
	import NodeRenderer from '$lib/components/NodeRenderer.svelte';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import type { Toast } from '$lib/ui.svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { shortcutToText } from '$lib/renderKey';
	import { keyEventMatches, type KeyboardShortcut } from '$lib/props';
	import { Kbd } from '../ui/kbd';

	type Props = {
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: any[]) => void;
		primaryAction?: UINode;
		secondaryAction?: UINode;
		actionPanel?: UINode;
		actions?: UINode[];
		navigationTitle?: string;
		toasts?: Map<number, Toast>;
		onToastAction: (toastId: number, actionType: 'primary' | 'secondary') => void;
		onHideToast: (toastId: number) => void;
	};
	let {
		uiTree,
		onDispatch,
		primaryAction,
		secondaryAction,
		actionPanel,
		actions,
		navigationTitle,
		toasts = new Map(),
		onToastAction
	}: Props = $props();

	let dropdownOpen = $state(false);

	const showActionPanelDropdown = $derived((actions?.length ?? 0) > 1);
	const toastToShow = $derived(Array.from(toasts.entries()).sort((a, b) => b[0] - a[0])[0]?.[1]);

	const toastActions = $derived.by(() => {
		const availableActions: {
			type: 'primary' | 'secondary';
			title: string;
			shortcut?: KeyboardShortcut;
		}[] = [];

		if (toastToShow?.primaryAction) {
			availableActions.push({ type: 'primary', ...toastToShow.primaryAction });
		}
		if (toastToShow?.secondaryAction) {
			availableActions.push({ type: 'secondary', ...toastToShow.secondaryAction });
		}
		return availableActions;
	});

	function handleKeydown(e: KeyboardEvent) {
		if (e.key.toLowerCase() === 't' && (e.ctrlKey || e.metaKey)) {
			if (toastActions.length > 0) {
				e.preventDefault();
				dropdownOpen = !dropdownOpen;
			}
		} else if (
			toastToShow?.primaryAction?.shortcut &&
			keyEventMatches(e, toastToShow.primaryAction.shortcut)
		) {
			handleActionSelect('primary');
		} else if (
			toastToShow?.secondaryAction?.shortcut &&
			keyEventMatches(e, toastToShow.secondaryAction.shortcut)
		) {
			handleActionSelect('secondary');
		}
	}

	function handleActionSelect(actionType: 'primary' | 'secondary') {
		if (toastToShow) {
			onToastAction(toastToShow.id, actionType);
		}
		dropdownOpen = false;
	}

	function handleDropdownKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			dropdownOpen = false;
			e.stopPropagation();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<footer class="bg-card flex h-12 shrink-0 items-center border-t px-4">
	{#if toastToShow}
		<DropdownMenu.Root bind:open={dropdownOpen}>
			<DropdownMenu.Trigger>
				{#snippet child({ props })}
					<div {...props} class="flex items-center gap-2">
						<div class="flex size-4 items-center justify-center">
							{#if toastToShow.style === 'ANIMATED'}
								<div
									class="size-4 animate-spin rounded-full border-2 border-white/30 border-t-white"
								></div>
							{:else if toastToShow.style === 'SUCCESS'}
								<div class="shadow-glow size-2 rounded-full bg-green-500 shadow-green-500"></div>
							{:else if toastToShow.style === 'FAILURE'}
								<div class="shadow-glow size-2 rounded-full bg-red-500 shadow-red-500"></div>
							{/if}
						</div>
						<div>
							<span>{toastToShow.title}</span>
							<span class="text-muted-foreground text-sm">{toastToShow.message}</span>
						</div>
						{#if toastToShow.primaryAction || toastToShow.secondaryAction}
							<Kbd>Ctrl + T</Kbd>
						{/if}
					</div>
				{/snippet}
			</DropdownMenu.Trigger>
			<DropdownMenu.Content side="top" align="start" onkeydown={handleDropdownKeydown} class="w-60">
				<DropdownMenu.Label>Toast Actions</DropdownMenu.Label>
				<DropdownMenu.Separator />
				{#each toastActions as action (action.type)}
					<DropdownMenu.Item onselect={() => handleActionSelect(action.type)}>
						{action.title}
						{#if action.shortcut}
							<DropdownMenu.Shortcut>{shortcutToText(action.shortcut)}</DropdownMenu.Shortcut>
						{/if}
					</DropdownMenu.Item>
				{/each}
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	{:else if navigationTitle}
		<div class="text-muted-foreground text-sm">{navigationTitle}</div>
	{/if}

	<div class="ml-auto flex items-center gap-2">
		{#if actionPanel}
			<div class="group flex items-center">
				{#if primaryAction}
					<NodeRenderer nodeId={primaryAction?.id} {uiTree} {onDispatch} displayAs="button" />
					{#if showActionPanelDropdown}
						<Separator
							orientation="vertical"
							class="!h-4 !w-0.5 !rounded-full transition-opacity group-hover:opacity-0"
						/>
					{/if}
				{/if}
				{#if showActionPanelDropdown}
					<NodeRenderer
						nodeId={actionPanel?.id}
						{uiTree}
						{onDispatch}
						primaryActionNodeId={primaryAction?.id}
						secondaryActionNodeId={secondaryAction?.id}
					/>
				{/if}
			</div>
		{/if}
	</div>
</footer>
