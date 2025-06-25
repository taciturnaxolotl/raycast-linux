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

	const VALID_PLACEHOLDERS_NO_CURSOR = new Set(['clipboard', 'date', 'time', 'datetime', 'day']);

	const parsedContent = $derived.by(() => {
		if (!content) return [];

		const parts: { text: string; highlightType: 'text' | 'valid-bracket' | 'invalid-bracket' }[] =
			[];
		let lastIndex = 0;
		const regex = /({([a-zA-Z_]+?)})/g;
		let match;
		let cursorFoundAndValid = false;

		while ((match = regex.exec(content)) !== null) {
			if (match.index > lastIndex) {
				parts.push({
					text: content.substring(lastIndex, match.index),
					highlightType: 'text'
				});
			}

			const placeholderName = match[2];
			let currentBracketHighlightType: 'valid-bracket' | 'invalid-bracket' = 'invalid-bracket';

			if (placeholderName === 'cursor') {
				if (!cursorFoundAndValid) {
					currentBracketHighlightType = 'valid-bracket';
					cursorFoundAndValid = true;
				} else {
					currentBracketHighlightType = 'invalid-bracket';
				}
			} else if (VALID_PLACEHOLDERS_NO_CURSOR.has(placeholderName)) {
				currentBracketHighlightType = 'valid-bracket';
			}

			parts.push({ text: '{', highlightType: currentBracketHighlightType });
			parts.push({ text: placeholderName, highlightType: 'text' });
			parts.push({ text: '}', highlightType: currentBracketHighlightType });

			lastIndex = regex.lastIndex;
		}

		if (lastIndex < content.length) {
			parts.push({ text: content.substring(lastIndex), highlightType: 'text' });
		}

		return parts;
	});

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
				<div class="grid w-full">
					<div
						aria-hidden="true"
						class="pointer-events-none col-start-1 row-start-1 min-h-32 w-full rounded-md border-transparent bg-transparent px-3 py-2 font-mono text-sm break-words whitespace-pre-wrap"
					>
						{#each parsedContent as part}
							<span
								class:text-blue-300={part.highlightType === 'valid-bracket'}
								class:text-red-400={part.highlightType === 'invalid-bracket'}
								class:text-foreground={part.highlightType === 'text'}
							>
								{part.text}
							</span>
						{/each}
						<span>&#x200B;</span>
					</div>
					<Textarea
						id="content"
						placeholder="Enter your snippet content... e.g. Hello {'{'}clipboard}!"
						bind:value={content}
						class="caret-foreground col-start-1 row-start-1 min-h-32 resize-none !bg-transparent font-mono text-transparent"
						spellcheck={false}
					/>
				</div>
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
