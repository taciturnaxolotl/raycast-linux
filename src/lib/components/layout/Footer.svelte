<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Kbd } from '../ui/kbd';
	import { keyEventMatches, type KeyboardShortcut } from '$lib/props';
	import { shortcutToText } from '$lib/renderKey';
	import type { Toast } from '$lib/ui.svelte';

	type Props = {
		toast: Toast;
		onToastAction: (toastId: number, actionType: 'primary' | 'secondary') => void;
	};
	let { toast, onToastAction }: Props = $props();

	let dropdownOpen = $state(false);

	const toastActions = $derived.by(() => {
		const availableActions: {
			type: 'primary' | 'secondary';
			title: string;
			shortcut?: KeyboardShortcut;
		}[] = [];

		if (toast?.primaryAction) {
			availableActions.push({ type: 'primary', ...toast.primaryAction });
		}
		if (toast?.secondaryAction) {
			availableActions.push({ type: 'secondary', ...toast.secondaryAction });
		}
		return availableActions;
	});

	function handleKeydown(e: KeyboardEvent) {
		if (e.key.toLowerCase() === 't' && (e.ctrlKey || e.metaKey)) {
			if (toastActions.length > 0) {
				e.preventDefault();
				dropdownOpen = !dropdownOpen;
			}
		} else if (toast?.primaryAction?.shortcut && keyEventMatches(e, toast.primaryAction.shortcut)) {
			handleActionSelect('primary');
		} else if (
			toast?.secondaryAction?.shortcut &&
			keyEventMatches(e, toast.secondaryAction.shortcut)
		) {
			handleActionSelect('secondary');
		}
	}

	function handleActionSelect(actionType: 'primary' | 'secondary') {
		if (toast) {
			onToastAction(toast.id, actionType);
		}
		dropdownOpen = false;
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<footer class="bg-card flex h-12 shrink-0 items-center border-t px-4">
	<DropdownMenu.Root bind:open={dropdownOpen}>
		<DropdownMenu.Trigger>
			{#snippet child({ props })}
				<div {...props} class="flex items-center gap-2">
					<div class="flex size-4 items-center justify-center">
						{#if toast.style === 'ANIMATED'}
							<div
								class="size-4 animate-spin rounded-full border-2 border-white/30 border-t-white"
							></div>
						{:else if toast.style === 'SUCCESS'}
							<div class="shadow-glow size-2 rounded-full bg-green-500 shadow-green-500"></div>
						{:else if toast.style === 'FAILURE'}
							<div class="shadow-glow size-2 rounded-full bg-red-500 shadow-red-500"></div>
						{/if}
					</div>
					<div>
						<span>{toast.title}</span>
						<span class="text-muted-foreground text-sm">{toast.message}</span>
					</div>
					{#if toast.primaryAction || toast.secondaryAction}
						<Kbd>Ctrl + T</Kbd>
					{/if}
				</div>
			{/snippet}
		</DropdownMenu.Trigger>
		<DropdownMenu.Content side="top" align="start" class="w-60">
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
</footer>
