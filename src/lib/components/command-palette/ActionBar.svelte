<script lang="ts">
	import type { UnifiedItem } from '$lib/command-palette.svelte';
	import ActionBar from '$lib/components/nodes/shared/ActionBar.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Kbd } from '$lib/components/ui/kbd';
	import ActionMenu from '$lib/components/nodes/shared/ActionMenu.svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { shortcutToText } from '$lib/renderKey';

	type Props = {
		selectedItem: UnifiedItem | undefined;
		actions: {
			handleEnter: () => Promise<void>;
			handleResetRanking: () => Promise<void>;
			handleCopyDeeplink: () => void;
			handleConfigureCommand: () => void;
			handleCopyAppName: () => void;
			handleCopyAppPath: () => void;
			handleHideApp: () => Promise<void>;
		};
	};

	let { selectedItem, actions: barActions }: Props = $props();
</script>

{#if selectedItem}
	<ActionBar>
		{#snippet primaryAction({ props })}
			{@const primaryActionText =
				selectedItem.type === 'app'
					? 'Open Application'
					: selectedItem.type === 'quicklink'
						? 'Open Quicklink'
						: 'Open Command'}
			<Button {...props} onclick={barActions?.handleEnter}>
				{primaryActionText}
				<Kbd>⏎</Kbd>
			</Button>
		{/snippet}
		{#snippet actions()}
			<ActionMenu>
				{#if selectedItem.type === 'plugin'}
					<DropdownMenu.Item onclick={barActions.handleResetRanking}
						>Reset Ranking</DropdownMenu.Item
					>
					<DropdownMenu.Separator />
					<DropdownMenu.Item onclick={barActions.handleCopyDeeplink}>
						Copy Deeplink
						<DropdownMenu.Shortcut>
							{shortcutToText({ key: 'c', modifiers: ['ctrl', 'shift'] })}
						</DropdownMenu.Shortcut>
					</DropdownMenu.Item>
					<DropdownMenu.Separator />
					<DropdownMenu.Item onclick={barActions.handleConfigureCommand}>
						Configure Command
						<DropdownMenu.Shortcut>
							{shortcutToText({ key: ',', modifiers: ['ctrl', 'shift'] })}
						</DropdownMenu.Shortcut>
					</DropdownMenu.Item>
				{:else if selectedItem.type === 'app'}
					<DropdownMenu.Item onclick={barActions.handleResetRanking}
						>Reset Ranking</DropdownMenu.Item
					>
					<DropdownMenu.Separator />
					<DropdownMenu.Item onclick={barActions.handleCopyAppName}>
						Copy Name
						<DropdownMenu.Shortcut>
							{shortcutToText({ key: '.', modifiers: ['ctrl'] })}
						</DropdownMenu.Shortcut>
					</DropdownMenu.Item>
					<DropdownMenu.Item onclick={barActions.handleCopyAppPath}>
						Copy Path
						<DropdownMenu.Shortcut>
							{shortcutToText({ key: '.', modifiers: ['ctrl', 'shift'] })}
						</DropdownMenu.Shortcut>
					</DropdownMenu.Item>
					<DropdownMenu.Separator />
					<DropdownMenu.Item onclick={barActions.handleHideApp}>
						Hide Application
						<DropdownMenu.Shortcut>
							{shortcutToText({ key: 'h', modifiers: ['ctrl'] })}
						</DropdownMenu.Shortcut>
					</DropdownMenu.Item>
				{/if}
			</ActionMenu>
		{/snippet}
	</ActionBar>
{/if}
