<script lang="ts">
	import type { Datum } from '$lib/store';
	import { Button } from '$lib/components/ui/button';
	import { Download, ArrowUpRight } from '@lucide/svelte';
	import Icon from '../Icon.svelte';
	import { openUrl } from '@tauri-apps/plugin-opener';
	import { Separator } from '../ui/separator';
	import * as Carousel from '$lib/components/ui/carousel/index.js';
	import ActionBar from '$lib/components/nodes/shared/ActionBar.svelte';
	import ActionMenu from '../nodes/shared/ActionMenu.svelte';
	import { DropdownMenuItem } from '../ui/dropdown-menu';

	type Props = {
		extension: Datum;
		isInstalling: boolean;
		onInstall: () => void;
		onOpenLightbox: (imageUrl: string) => void;
	};

	let { extension, isInstalling, onInstall, onOpenLightbox }: Props = $props();

	function formatTimeAgo(timestamp: number) {
		const date = new Date(timestamp * 1000);
		const now = new Date();
		const seconds = Math.floor((now.getTime() - date.getTime()) / 1000);
		let interval = seconds / 31536000;
		if (interval > 1) {
			const years = Math.floor(interval);
			return `${years} year${years > 1 ? 's' : ''} ago`;
		}
		interval = seconds / 2592000;
		if (interval > 1) {
			const months = Math.floor(interval);
			return `${months} month${months > 1 ? 's' : ''} ago`;
		}
		interval = seconds / 604800;
		if (interval > 1) {
			const weeks = Math.floor(interval);
			return `${weeks} week${weeks > 1 ? 's' : ''} ago`;
		}
		interval = seconds / 86400;
		if (interval > 1) {
			const days = Math.floor(interval);
			return `${days} day${days > 1 ? 's' : ''} ago`;
		}
		interval = seconds / 3600;
		if (interval > 1) {
			const hours = Math.floor(interval);
			return `${hours} hour${hours > 1 ? 's' : ''} ago`;
		}
		interval = seconds / 60;
		if (interval > 1) {
			const minutes = Math.floor(interval);
			return `${minutes} minute${minutes > 1 ? 's' : ''} ago`;
		}
		return `${Math.floor(seconds)} second${seconds !== 1 ? 's' : ''} ago`;
	}
</script>

<div class="grow overflow-y-auto">
	<div class="mx-auto max-w-4xl p-6">
		<div class="flex items-center gap-4">
			<Icon
				icon={extension.icons.light ? { source: extension.icons.light, mask: 'Circle' } : undefined}
				class="size-16"
			/>
			<div>
				<h1 class="text-3xl font-bold">{extension.title}</h1>
				<div class="mt-2 flex items-center gap-4 text-gray-400">
					<div class="flex items-center gap-1.5">
						<Icon
							icon={extension.author.avatar
								? { source: extension.author.avatar, mask: 'Circle' }
								: undefined}
							class="size-5"
						/>
						<span>{extension.author.name}</span>
					</div>
					<div class="flex items-center gap-1.5">
						<Download class="size-4" />
						<span>{extension.download_count.toLocaleString()} Installs</span>
					</div>
					{#if extension.categories?.includes('AI Extensions')}
						<div
							class="flex items-center gap-1.5 rounded bg-purple-500/20 px-1.5 py-0.5 text-xs text-purple-300"
						>
							<Icon icon="command-symbol-16" class="size-3" />
							<span>AI Extension</span>
						</div>
					{/if}
				</div>
			</div>
		</div>

		{#if extension.metadata_count > 0}
			<Carousel.Root class="mt-8 w-full">
				<Carousel.Content>
					{#each Array(extension.metadata_count) as _, i}
						<Carousel.Item class="basis-1/3">
							{@const imageUrl = `${extension.readme_assets_path}metadata/${extension.name}-${i + 1}.png`}
							<button class="w-full cursor-pointer" onclick={() => onOpenLightbox(imageUrl)}>
								<img
									src={imageUrl}
									alt={`Screenshot ${i + 1} for ${extension.title}`}
									class="aspect-video w-full rounded-lg bg-white/5 object-cover"
									loading="lazy"
								/>
							</button>
						</Carousel.Item>
					{/each}
				</Carousel.Content>
				<Carousel.Previous />
				<Carousel.Next />
			</Carousel.Root>
		{/if}

		<div class="mt-8 grid grid-cols-[1fr_auto_1fr] gap-x-8">
			<div class="flex flex-col gap-4">
				<div>
					<h2 class="text-muted-foreground mb-1 text-xs font-medium uppercase">Description</h2>
					<p>{extension.description}</p>
				</div>

				<Separator />

				<div>
					<h2 class="text-muted-foreground mb-2 text-xs font-medium uppercase">Commands</h2>
					<div class="flex flex-col gap-4">
						{#each extension.commands as command (command.id)}
							<div class="flex items-start gap-3">
								<Icon
									icon={command.icons.light
										? { source: command.icons.light, mask: 'Circle' }
										: undefined}
									class="mt-1 size-5"
								/>
								<div>
									<p class="mb-1 text-sm font-medium">{command.title}</p>
									<p class="text-muted-foreground text-xs">{command.description}</p>
								</div>
							</div>
						{/each}
					</div>
				</div>
			</div>
			<Separator orientation="vertical" />
			<div class="space-y-8">
				<div>
					<h2 class="text-muted-foreground mb-1 text-xs font-medium uppercase">README</h2>
					<Button
						variant="ghost"
						class="-mx-3 w-full justify-between"
						onclick={() => extension?.readme_url && openUrl(extension.readme_url)}
					>
						Open README <ArrowUpRight class="text-muted-foreground size-4" />
					</Button>
				</div>
				<div>
					<h3 class="text-muted-foreground mb-1 text-xs font-medium uppercase">Last updated</h3>
					<p>{formatTimeAgo(extension.updated_at)}</p>
				</div>
				<div>
					<h3 class="text-muted-foreground mb-1 text-xs font-medium uppercase">Contributors</h3>
					<div class="flex flex-wrap gap-2">
						{#each extension.contributors as contributor (contributor.handle)}
							<a
								href={`https://github.com/${contributor.github_handle}`}
								target="_blank"
								class="flex items-center gap-2"
								rel="noopener noreferrer"
							>
								<Icon
									icon={contributor.avatar
										? { source: contributor.avatar, mask: 'Circle' }
										: undefined}
									class="size-6"
								/>
							</a>
						{/each}
					</div>
				</div>
				{#if extension.categories?.length > 0}
					<div>
						<h3 class="text-muted-foreground mb-1 text-xs font-medium uppercase">Categories</h3>
						<div class="flex flex-wrap gap-1.5">
							{#each extension.categories as category}
								<span
									class="rounded-full bg-blue-900/50 px-2 py-0.5 text-xs font-semibold text-blue-300"
								>
									{category}
								</span>
							{/each}
						</div>
					</div>
				{/if}
				<div>
					<h3 class="text-muted-foreground mb-1 text-xs font-medium uppercase">Source Code</h3>
					<Button
						variant="ghost"
						class="-mx-3 w-full justify-between"
						onclick={() => extension?.source_url && openUrl(extension.source_url)}
					>
						View Code <ArrowUpRight class="text-muted-foreground size-4" />
					</Button>
				</div>
			</div>
		</div>
	</div>
</div>

<ActionBar
	title={extension.title}
	icon={extension.icons.light ? { source: extension.icons.light, mask: 'Circle' } : undefined}
>
	{#snippet primaryAction({ props })}
		<Button {...props} onclick={onInstall} disabled={isInstalling}>
			{isInstalling ? 'Installing...' : 'Install Extension'}
		</Button>
	{/snippet}
	{#snippet actions()}
		<ActionMenu>
			<DropdownMenuItem onclick={onInstall} disabled={isInstalling}>
				{isInstalling ? 'Installing...' : 'Install Extension'}
			</DropdownMenuItem>
		</ActionMenu>
	{/snippet}
</ActionBar>
