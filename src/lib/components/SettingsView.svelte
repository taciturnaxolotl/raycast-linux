<script lang="ts">
	import type { PluginInfo, Preference } from '@raycast-linux/protocol';
	import { Input } from '$lib/components/ui/input';
	import Icon from '$lib/components/Icon.svelte';
	import { Label } from './ui/label';
	import { Checkbox } from './ui/checkbox';
	import * as Select from './ui/select';
	import BaseList from './BaseList.svelte';
	import { Button } from './ui/button';

	type Props = {
		plugins: PluginInfo[];
		onBack: () => void;
		onSavePreferences: (pluginName: string, values: Record<string, unknown>) => void;
		onGetPreferences: (pluginName: string) => void;
		currentPreferences: Record<string, unknown>;
	};

	type DisplayListItem = {
		id: string;
		itemType: 'item';
		type: 'extension' | 'command';
		data: PluginInfo;
		isLastInGroup: boolean;
	};

	let { plugins, onBack, onSavePreferences, onGetPreferences, currentPreferences }: Props =
		$props();

	let selectedIndex = $state(0);
	let preferenceValues = $state<Record<string, unknown>>({});
	let searchText = $state('');

	const displayItems = $derived.by(() => {
		const items: DisplayListItem[] = [];
		const extensions = new Map<string, PluginInfo[]>();

		for (const plugin of plugins) {
			if (!extensions.has(plugin.pluginName)) {
				extensions.set(plugin.pluginName, []);
			}
			extensions.get(plugin.pluginName)!.push(plugin);
		}

		const lowerSearchText = searchText.toLowerCase();

		for (const [, commands] of extensions.entries()) {
			const firstCommand = commands[0];
			const hasGlobalPrefs = firstCommand.preferences && firstCommand.preferences.length > 0;
			const commandsWithPrefs = commands.filter(
				(p) => p.commandPreferences && p.commandPreferences.length > 0
			);

			if (!hasGlobalPrefs && commandsWithPrefs.length === 0) continue;

			const extensionMatches =
				firstCommand.pluginTitle.toLowerCase().includes(lowerSearchText) ||
				(firstCommand.description ?? '').toLowerCase().includes(lowerSearchText);

			const matchingCommands = commandsWithPrefs.filter(
				(c) =>
					c.title.toLowerCase().includes(lowerSearchText) ||
					(c.description ?? '').toLowerCase().includes(lowerSearchText)
			);

			if (searchText && !extensionMatches && matchingCommands.length === 0) {
				continue;
			}

			items.push({
				id: firstCommand.pluginName,
				type: 'extension',
				itemType: 'item',
				data: firstCommand,
				isLastInGroup: false
			});

			const commandsToShow = extensionMatches ? commandsWithPrefs : matchingCommands;
			commandsToShow.forEach((command, index) => {
				items.push({
					id: `${command.pluginName}/${command.commandName}`,
					type: 'command',
					itemType: 'item',
					data: command,
					isLastInGroup: index === commandsToShow.length - 1
				});
			});
		}

		return items;
	});

	const selectedItem = $derived(displayItems[selectedIndex]);

	const preferencesToShow = $derived.by(() => {
		if (!selectedItem) return [];
		return selectedItem.type === 'extension'
			? (selectedItem.data.preferences ?? [])
			: (selectedItem.data.commandPreferences ?? []);
	});

	$effect(() => {
		if (selectedItem) {
			onGetPreferences(selectedItem.data.pluginName);
		}
	});

	$effect(() => {
		preferenceValues = { ...currentPreferences };
	});

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			event.preventDefault();
			onBack();
		}
	}

	function handleSave() {
		if (selectedItem) {
			onSavePreferences(selectedItem.data.pluginName, preferenceValues);
		}
	}

	function handlePreferenceChange(prefName: string, value: unknown) {
		preferenceValues = { ...preferenceValues, [prefName]: value };
	}

	function getPreferenceValue(pref: Preference): unknown {
		return preferenceValues[pref.name] ?? pref.default ?? '';
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<main class="bg-background text-foreground flex h-screen">
	<div class="flex w-80 flex-col border-r">
		<header class="flex h-12 shrink-0 items-center border-b px-2">
			<button onclick={onBack} class="hover:bg-accent mr-2 rounded p-1">
				<Icon icon="chevron-left-16" class="size-4" />
			</button>
			<Input
				class="rounded-none border-none !bg-transparent"
				placeholder="Filter by name..."
				bind:value={searchText}
			/>
		</header>

		<div class="flex-1 overflow-y-auto">
			<BaseList items={displayItems} bind:selectedIndex onenter={() => {}} autofocus>
				{#snippet itemSnippet({ item, isSelected, onclick })}
					<div class="relative">
						<button
							type="button"
							class="hover:bg-accent/50 flex w-full items-center gap-3 py-3 text-left"
							class:bg-accent={isSelected}
							class:pl-4={item.type === 'extension'}
							class:pl-12={item.type === 'command'}
							{onclick}
						>
							<div class="flex size-8 shrink-0 items-center justify-center">
								<Icon icon={item.data.icon || 'app-window-16'} class="size-5" />
							</div>
							<div class="flex min-w-0 flex-col">
								<span class="truncate text-sm font-medium">
									{item.type === 'extension' ? item.data.pluginTitle : item.data.title}
								</span>
								<span class="text-muted-foreground truncate text-xs">
									{item.type === 'extension' ? 'Extension' : 'Command'}
								</span>
							</div>
						</button>
					</div>
				{/snippet}
			</BaseList>
		</div>
	</div>

	<div class="flex flex-1 flex-col overflow-y-auto">
		{#if selectedItem}
			<div class="p-6">
				<div class="mb-6 flex items-start justify-between">
					<div>
						<h2 class="text-lg font-medium">
							{selectedItem.type === 'extension'
								? selectedItem.data.pluginTitle
								: selectedItem.data.title}
						</h2>
						<p class="text-muted-foreground mt-1 text-sm">
							{selectedItem.type === 'extension'
								? 'These settings apply to the entire extension.'
								: 'These settings apply only to this command.'}
						</p>
					</div>

					<Button onclick={handleSave}>Save</Button>
				</div>
				{#if preferencesToShow.length > 0}
					<div class="max-w-md space-y-6">
						{#each preferencesToShow as pref}
							<div class="space-y-2">
								<label class="text-sm font-medium">
									{pref.title}
									{#if pref.required}<span class="text-red-500">*</span>{/if}
								</label>

								{#if pref.description}
									<p class="text-muted-foreground text-xs">{pref.description}</p>
								{/if}

								{#if pref.type === 'textfield'}
									<Input
										value={getPreferenceValue(pref) as string}
										onchange={(e) =>
											handlePreferenceChange(pref.name, (e.target as HTMLInputElement)?.value)}
										placeholder={pref.default as string}
									/>
								{:else if pref.type === 'checkbox'}
									<Label class="flex items-center gap-2">
										<Checkbox
											checked={getPreferenceValue(pref) as boolean}
											onCheckedChange={(checked) => handlePreferenceChange(pref.name, checked)}
										/>
										<span class="text-sm">{pref.title}</span>
									</Label>
								{:else if pref.type === 'dropdown' && pref.data}
									<Select.Root
										value={getPreferenceValue(pref) as string}
										onValueChange={(value) => handlePreferenceChange(pref.name, value)}
										type="single"
									>
										<Select.Trigger
											class="bg-background border-border w-full rounded border px-3 py-2 text-sm"
										>
											{@const preference = pref.data.find(
												(option) => option.value === getPreferenceValue(pref)
											)}
											{preference?.title ?? pref.default}
										</Select.Trigger>
										<Select.Content>
											{#each pref.data as option}
												<Select.Item value={option.value}>{option.title}</Select.Item>
											{/each}
										</Select.Content>
									</Select.Root>
								{:else if pref.type === 'directory'}
									<Input
										value={getPreferenceValue(pref) as string}
										onchange={(e) =>
											handlePreferenceChange(pref.name, (e.target as HTMLInputElement)?.value)}
										placeholder={pref.default as string}
									/>
								{/if}
							</div>
						{/each}
					</div>
				{:else}
					<div class="flex h-full items-center justify-center">
						<div class="text-center">
							<p class="text-muted-foreground">No preferences to configure.</p>
						</div>
					</div>
				{/if}
			</div>
		{:else}
			<div class="flex h-full flex-1 items-center justify-center">
				<div class="text-center">
					<p class="text-muted-foreground">Select an item to configure its settings</p>
				</div>
			</div>
		{/if}
	</div>
</main>
