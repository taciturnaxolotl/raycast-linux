<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Switch } from './ui/switch';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import PasswordInput from './PasswordInput.svelte';
	import { Model } from '../../../sidecar/src/api/ai';

	let aiEnabled = $state(false);
	let apiKey = $state('');
	let modelAssociations = $state<Record<string, string>>({});
	let isApiKeySet = $state(false);

	async function loadSettings() {
		isApiKeySet = await invoke('is_ai_api_key_set');
		// TODO: Load aiEnabled and modelAssociations from storage
	}

	async function saveSettings() {
		if (apiKey) {
			await invoke('set_ai_api_key', { key: apiKey });
		}
		// TODO: Save aiEnabled and modelAssociations to storage
		await loadSettings();
	}

	async function clearApiKey() {
		await invoke('clear_ai_api_key');
		apiKey = '';
		await loadSettings();
	}

	onMount(loadSettings);
</script>

<div class="mx-auto max-w-screen-md space-y-6 p-6">
	<div class="space-y-2">
		<h3 class="text-lg font-medium">General AI Settings</h3>
		<div class="flex items-center space-x-2">
			<Switch bind:checked={aiEnabled} id="ai-enabled" />
			<label for="ai-enabled" class="text-sm font-medium"> Enable AI Features </label>
		</div>
	</div>

	<div class="space-y-2">
		<h3 class="text-lg font-medium">API Key</h3>
		<p class="text-muted-foreground text-sm">
			Your OpenRouter API key is stored securely in your system's keychain.
		</p>
		<div class="flex items-center gap-2">
			<PasswordInput
				bind:value={apiKey}
				placeholder={isApiKeySet ? '••••••••••••' : 'Enter your OpenRouter API key'}
				class="flex-grow"
			/>
			{#if isApiKeySet}
				<Button variant="destructive" onclick={clearApiKey}>Clear</Button>
			{/if}
		</div>
	</div>

	<div class="space-y-2">
		<h3 class="text-lg font-medium">Model Associations</h3>
		<p class="text-muted-foreground text-sm">
			Associate Raycast AI models with specific models available through OpenRouter.
		</p>
		<div class="grid grid-cols-[auto_1fr] items-center gap-4">
			{#each Object.entries(Model) as [raycastModel, openRouterModel] (raycastModel)}
				<span class="text-sm font-medium">{raycastModel}</span>
				<Input
					value={modelAssociations[raycastModel] ?? openRouterModel}
					onchange={(e) => {
						modelAssociations[raycastModel] = (e.target as HTMLInputElement)?.value;
					}}
					placeholder={openRouterModel}
					class="w-full"
				/>
			{/each}
		</div>
	</div>
	<div class="flex justify-end">
		<Button onclick={saveSettings}>Save AI Settings</Button>
	</div>
</div>
