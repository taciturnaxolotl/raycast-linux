<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Popover from '$lib/components/ui/popover';
	import * as Command from '$lib/components/ui/command';
	import { ChevronsUpDown, Check } from '@lucide/svelte';
	import { tick } from 'svelte';

	type Props = {
		allCategories: string[];
		selectedCategory: string;
	};

	let { allCategories, selectedCategory = $bindable() }: Props = $props();
	let categoryPopoverOpen = $state(false);
</script>

<Popover.Root bind:open={categoryPopoverOpen}>
	<Popover.Trigger>
		{#snippet child({ props })}
			<Button
				variant="ghost"
				role="combobox"
				aria-expanded={categoryPopoverOpen}
				class="w-48 justify-between"
				{...props}
			>
				{selectedCategory}
				<ChevronsUpDown class="ml-2 size-4 shrink-0 opacity-50" />
			</Button>
		{/snippet}
	</Popover.Trigger>
	<Popover.Content class="w-48 p-0">
		<Command.Root>
			<Command.Input placeholder="Search category..." />
			<Command.Empty>No category found.</Command.Empty>
			<Command.List>
				{#each allCategories as category}
					<Command.Item
						value={category}
						onSelect={() => {
							selectedCategory = category;
							categoryPopoverOpen = false;
						}}
					>
						<Check class={selectedCategory !== category ? 'text-transparent' : ''} />
						{category}
					</Command.Item>
				{/each}
			</Command.List>
		</Command.Root>
	</Popover.Content>
</Popover.Root>
