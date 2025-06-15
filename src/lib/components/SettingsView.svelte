<script lang="ts">
	import type { PluginInfo, Preference } from '@raycast-linux/protocol';
	import { Input } from '$lib/components/ui/input';
	import Icon from '$lib/components/Icon.svelte';
	import { Label } from './ui/label';
	import { Checkbox } from './ui/checkbox';
	import * as Select from './ui/select';

	type Props = {
		plugins: PluginInfo[];
		onBack: () => void;
		onSavePreferences: (pluginName: string, values: Record<string, unknown>) => void;
		onGetPreferences: (pluginName: string) => void;
		currentPreferences: Record<string, unknown>;
	};

	let { plugins, onBack, onSavePreferences, onGetPreferences, currentPreferences }: Props =
		$props();

	let selectedPluginIndex = $state(0);
	let preferenceValues = $state<Record<string, unknown>>({});

	const selectedPlugin = $derived(plugins[selectedPluginIndex]);
	const pluginsWithPreferences = $derived(
		plugins.filter((p) => p.preferences && p.preferences.length > 0)
	);

	$effect(() => {
		if (selectedPlugin) {
			onGetPreferences(selectedPlugin.pluginName);
		}
	});

	$effect(() => {
		preferenceValues = { ...currentPreferences };
	});

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			event.preventDefault();
			onBack();
		} else if (event.key === 'ArrowUp') {
			event.preventDefault();
			selectedPluginIndex = Math.max(0, selectedPluginIndex - 1);
		} else if (event.key === 'ArrowDown') {
			event.preventDefault();
			selectedPluginIndex = Math.min(pluginsWithPreferences.length - 1, selectedPluginIndex + 1);
		}
	}

	function handleSave() {
		if (selectedPlugin) {
			onSavePreferences(selectedPlugin.pluginName, preferenceValues);
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
		<header class="flex h-12 shrink-0 items-center border-b px-4">
			<button onclick={onBack} class="hover:bg-accent mr-3 rounded p-1">
				<Icon icon="chevron-left-16" class="size-4" />
			</button>
			<h1 class="font-medium">Extension Settings</h1>
		</header>

		<div class="flex-1 overflow-y-auto">
			{#each pluginsWithPreferences as plugin, index}
				<button
					type="button"
					class="hover:bg-accent/50 flex w-full items-center gap-3 px-4 py-3 text-left"
					class:bg-accent={selectedPluginIndex === index}
					onclick={() => (selectedPluginIndex = index)}
				>
					<div class="flex size-8 shrink-0 items-center justify-center">
						<Icon icon={plugin.icon || 'app-window-16'} class="size-5" />
					</div>
					<div class="flex flex-col">
						<span class="text-sm font-medium">{plugin.title}</span>
						<span class="text-muted-foreground text-xs">{plugin.pluginName}</span>
					</div>
				</button>
			{/each}
		</div>
	</div>

	<div class="flex flex-1 flex-col">
		{#if selectedPlugin && selectedPlugin.preferences}
			<header class="flex h-12 shrink-0 items-center justify-between border-b px-6">
				<div>
					<h2 class="font-medium">{selectedPlugin.title}</h2>
					<p class="text-muted-foreground text-sm">{selectedPlugin.description}</p>
				</div>
				<button
					onclick={handleSave}
					class="bg-primary text-primary-foreground hover:bg-primary/90 rounded px-4 py-2 text-sm"
				>
					Save
				</button>
			</header>

			<div class="flex-1 overflow-y-auto p-6">
				<div class="max-w-md space-y-6">
					{#each selectedPlugin.preferences as pref}
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
								<Label class="flex items-center">
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
			</div>
		{:else}
			<div class="flex flex-1 items-center justify-center">
				<div class="text-center">
					<p class="text-muted-foreground">Select a plugin to configure its settings</p>
				</div>
			</div>
		{/if}
	</div>
</main>
