<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import Icon from '$lib/components/Icon.svelte';
	import { ArrowLeft, Save } from '@lucide/svelte';
	import { uiStore } from '$lib/ui.svelte';

	type Props = {
		onBack: () => void;
		onSave: () => void;
	};

	let { onBack, onSave }: Props = $props();

	let name = $state('');
	let keyword = $state('');
	let content = $state('');
	let error = $state('');

	async function handleSave() {
		if (!name.trim() || !keyword.trim() || !content.trim()) {
			error = 'All fields are required.';
			return;
		}
		error = '';

		try {
			await invoke('create_snippet', { name, keyword, content });
			uiStore.toasts.set(Date.now(), {
				id: Date.now(),
				title: 'Snippet Created',
				style: 'SUCCESS'
			});
			onSave();
		} catch (e) {
			const errorMessage = e instanceof Error ? e.message : String(e);
			error = errorMessage;
			console.error('Failed to create snippet:', e);
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && (event.metaKey || event.ctrlKey)) {
			event.preventDefault();
			handleSave();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<main class="bg-background text-foreground flex h-screen flex-col">
	<header class="flex h-12 shrink-0 items-center border-b px-2">
		<Button variant="ghost" size="icon" onclick={onBack}>
			<ArrowLeft class="size-5" />
		</Button>
		<div class="flex items-center gap-3 px-2">
			<Icon icon="snippets-16" class="size-6" />
			<h1 class="text-lg font-medium">Create Snippet</h1>
		</div>
	</header>
	<div class="grow overflow-y-auto p-6">
		<div class="mx-auto max-w-xl space-y-6">
			<div class="grid grid-cols-[120px_1fr] items-center gap-4">
				<label for="name" class="text-right text-sm text-gray-400">Name</label>
				<Input id="name" placeholder="Snippet name" bind:value={name} />
			</div>
			<div class="grid grid-cols-[120px_1fr] items-center gap-4">
				<label for="keyword" class="text-right text-sm text-gray-400">Keyword</label>
				<Input id="keyword" placeholder="!email" bind:value={keyword} />
			</div>

			<div class="grid grid-cols-[120px_1fr] items-start gap-4">
				<label for="content" class="pt-2 text-right text-sm text-gray-400">Snippet</label>
				<Textarea
					id="content"
					placeholder="Enter your snippet content here..."
					bind:value={content}
					class="min-h-32 font-mono"
				/>
			</div>

			{#if error}
				<p class="text-center text-red-500">{error}</p>
			{/if}
		</div>
	</div>
	<footer class="bg-card flex h-12 shrink-0 items-center justify-end border-t px-4">
		<Button onclick={handleSave}><Save class="mr-2 size-4" /> Create Snippet</Button>
	</footer>
</main>
