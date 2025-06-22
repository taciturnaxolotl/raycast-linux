<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import * as Select from '$lib/components/ui/select';
	import Icon from '$lib/components/Icon.svelte';
	import { ArrowLeft, Save } from '@lucide/svelte';
	import { quicklinksStore, type Quicklink } from '$lib/quicklinks.svelte';

	type AppInfo = {
		name: string;
		exec: string;
		icon_path?: string;
	};

	type Props = {
		quicklink?: Quicklink;
		onBack: () => void;
		onSave: () => void;
	};

	let { quicklink, onBack, onSave }: Props = $props();

	let name = $state(quicklink?.name ?? '');
	let link = $state(quicklink?.link ?? '');
	let application = $state(quicklink?.application ?? 'Default');
	let icon = $state(quicklink?.icon ?? 'link-16');

	let applications = $state<AppInfo[]>([]);
	let error = $state('');

	onMount(async () => {
		try {
			applications = (await invoke('get_installed_apps')) as AppInfo[];
		} catch (e) {
			console.error('Failed to fetch installed apps:', e);
		}
	});

	async function handleSave() {
		if (!name.trim()) {
			error = 'Name cannot be empty';
			return;
		}
		error = '';

		const data = {
			name,
			link,
			application: application === 'Default' ? undefined : application,
			icon: icon === 'link-16' ? undefined : icon
		};

		try {
			if (quicklink) {
				await quicklinksStore.update(quicklink.id, data);
			} else {
				await quicklinksStore.create(data);
			}
			onSave();
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
		}
	}
</script>

<main class="bg-background text-foreground flex h-screen flex-col">
	<header class="flex h-12 shrink-0 items-center border-b px-2">
		<Button variant="ghost" size="icon" onclick={onBack}>
			<ArrowLeft class="size-5" />
		</Button>
		<div class="flex items-center gap-3 px-2">
			<Icon icon="link-16" class="size-6" />
			<h1 class="text-lg font-medium">{quicklink ? 'Edit Quicklink' : 'Create Quicklink'}</h1>
		</div>
	</header>
	<div class="grow overflow-y-auto p-6">
		<div class="mx-auto max-w-xl space-y-6">
			<div class="grid grid-cols-[120px_1fr] items-center gap-4">
				<label for="name" class="text-right text-sm text-gray-400">Name</label>
				<Input id="name" placeholder="Quicklink name" bind:value={name} />
			</div>

			<div class="grid grid-cols-[120px_1fr] items-start gap-4">
				<label for="link" class="pt-2 text-right text-sm text-gray-400">Link</label>
				<div>
					<Textarea
						id="link"
						placeholder="https://google.com/search?q={'{argument}'}"
						bind:value={link}
					/>
					<p class="text-muted-foreground mt-1 text-xs">
						Include <span class="text-foreground font-mono">{'{argument}'}</span> for context like the
						selected or copied text in the link.
					</p>
				</div>
			</div>

			<div class="grid grid-cols-[120px_1fr] items-center gap-4">
				<label for="open-with" class="text-right text-sm text-gray-400">Open With</label>
				<Select.Root bind:value={application} type="single">
					<Select.Trigger id="open-with" class="w-full">
						{@const selectedApp = applications.find((a) => a.exec === application)}
						{selectedApp?.name ?? 'Default'}
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="Default">Default</Select.Item>
						{#each applications as app (app.exec)}
							<Select.Item value={app.exec}>{app.name}</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>

			<div class="grid grid-cols-[120px_1fr] items-center gap-4">
				<label for="icon" class="text-right text-sm text-gray-400">Icon</label>
				<Input id="icon" placeholder="link-16" bind:value={icon} />
			</div>

			{#if error}
				<p class="text-center text-red-500">{error}</p>
			{/if}
		</div>
	</div>
	<footer class="bg-card flex h-12 shrink-0 items-center justify-between border-t px-4">
		<div class="flex items-center gap-2">
			<Icon icon="link-16" class="size-5" />
			<span class="text-sm font-medium">Create Quicklink</span>
		</div>
		<Button onclick={handleSave}><Save class="mr-2 size-4" /> Save Quicklink</Button>
	</footer>
</main>
