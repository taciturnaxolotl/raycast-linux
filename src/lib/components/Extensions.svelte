<script lang="ts">
	import type { Datum } from '$lib/store';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { ArrowLeft } from '@lucide/svelte';
	import Icon from './Icon.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import ExtensionListView from './extensions/ExtensionListView.svelte';
	import ExtensionDetailView from './extensions/ExtensionDetailView.svelte';
	import ImageLightbox from './extensions/ImageLightbox.svelte';
	import CategoryFilter from './extensions/CategoryFilter.svelte';
	import { extensionsStore } from './extensions/store.svelte';

	type Props = {
		onBack: () => void;
		onInstall: () => void;
	};

	let { onBack, onInstall }: Props = $props();

	let selectedExtension = $state<Datum | null>(null);
	let expandedImageUrl = $state<string | null>(null);
	let isInstalling = $state(false);
	let scrollContainer = $state<HTMLElement | null>(null);

	$effect(() => {
		const container = scrollContainer;
		if (!container) return;
		const handleScroll = () => {
			if (container.scrollHeight - container.scrollTop - container.clientHeight < 500) {
				extensionsStore.loadMore();
			}
		};
		container.addEventListener('scroll', handleScroll);
		return () => container.removeEventListener('scroll', handleScroll);
	});

	function handleGlobalKeyDown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			e.preventDefault();
			if (expandedImageUrl) {
				expandedImageUrl = null;
			} else if (selectedExtension) {
				selectedExtension = null;
			} else {
				onBack();
			}
			return;
		}
	}

	async function handleInstall() {
		if (!selectedExtension || isInstalling) return;
		isInstalling = true;
		try {
			await invoke('install_extension', {
				downloadUrl: selectedExtension.download_url,
				slug: selectedExtension.name
			});
			onInstall();
		} catch (e) {
			console.error('Installation failed', e);
		} finally {
			isInstalling = false;
		}
	}
</script>

<svelte:window onkeydown={handleGlobalKeyDown} />

<main class="bg-background text-foreground flex h-screen flex-col">
	<header class="flex h-12 shrink-0 items-center border-b px-2">
		<Button
			variant="ghost"
			size="icon"
			onclick={() => (selectedExtension ? (selectedExtension = null) : onBack())}
		>
			<ArrowLeft class="size-5" />
		</Button>
		{#if selectedExtension}
			<div class="flex items-center gap-3 px-2">
				<Icon
					icon={selectedExtension.icons.light
						? { source: selectedExtension.icons.light, mask: 'Circle' }
						: undefined}
					class="size-6"
				/>
				<h1 class="text-lg font-medium">{selectedExtension.title}</h1>
			</div>
		{:else}
			<Input
				class="rounded-none border-none !bg-transparent pr-0"
				placeholder="Search Store for extensions..."
				bind:value={extensionsStore.searchText}
				autofocus
			/>
			<CategoryFilter />
		{/if}
	</header>

	{#if selectedExtension}
		<ExtensionDetailView
			extension={selectedExtension}
			{isInstalling}
			onInstall={handleInstall}
			onOpenLightbox={(imageUrl) => (expandedImageUrl = imageUrl)}
		/>
	{:else}
		<div class="grow overflow-y-auto" role="listbox" tabindex={-1} bind:this={scrollContainer}>
			<ExtensionListView onSelect={(ext) => (selectedExtension = ext)} />
		</div>
	{/if}
</main>

{#if expandedImageUrl}
	<ImageLightbox imageUrl={expandedImageUrl} onClose={() => (expandedImageUrl = null)} />
{/if}
