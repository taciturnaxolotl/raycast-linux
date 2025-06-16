<script lang="ts">
	import type { PluginInfo } from '@raycast-linux/protocol';
	import Icon from '$lib/components/Icon.svelte';
	import { Input } from '$lib/components/ui/input';
	import { create, all } from 'mathjs';
	import { ArrowRight } from '@lucide/svelte';
	import { writeText } from '@tauri-apps/plugin-clipboard-manager';

	type Props = {
		plugins: PluginInfo[];
		onRunPlugin: (plugin: PluginInfo) => void;
	};

	let { plugins, onRunPlugin }: Props = $props();

	let searchText = $state('');
	let selectedIndex = $state(0);

	const math = create(all);
	let mathResult = $state<string | null>(null);
	let inputWords = $state<string | null>(null);
	let resultWords = $state<string | null>(null);

	const filteredPlugins = $derived.by(() => {
		if (!searchText) return plugins;
		const lowerCaseSearch = searchText.toLowerCase();
		return plugins.filter(
			(p: PluginInfo) =>
				p.title.toLowerCase().includes(lowerCaseSearch) ||
				p.description?.toLowerCase().includes(lowerCaseSearch) ||
				p.pluginName.toLowerCase().includes(lowerCaseSearch)
		);
	});

	$effect(() => {
		if (!searchText.trim()) {
			mathResult = null;
			inputWords = null;
			resultWords = null;
			return;
		}

		try {
			const result = math.evaluate(searchText.trim());

			if (typeof result === 'function' || typeof result === 'undefined') {
				mathResult = null;
				inputWords = null;
				resultWords = null;
				return;
			}

			let resultString = math.format(result, { precision: 14 });

			if (resultString === searchText.trim()) {
				mathResult = null;
				inputWords = null;
				resultWords = null;
				return;
			}

			mathResult = resultString;
			inputWords = isFinite(Number(searchText.trim()))
				? numberToWords(searchText.trim())
				: 'Expression';
			resultWords = isFinite(Number(resultString))
				? numberToWords(resultString)
				: math.typeOf(result);
		} catch (error) {
			mathResult = null;
			inputWords = null;
			resultWords = null;
		}
	});

	const hasMathResult = $derived(!!mathResult);

	$effect(() => {
		if (hasMathResult) {
			selectedIndex = 0;
		}
	});

	$effect(() => {
		const totalItems = (hasMathResult ? 1 : 0) + filteredPlugins.length;
		if (selectedIndex >= totalItems) {
			selectedIndex = Math.max(0, totalItems - 1);
		}
	});

	function handleKeydown(event: KeyboardEvent) {
		const totalItems = (hasMathResult ? 1 : 0) + filteredPlugins.length;
		if (totalItems === 0) return;

		if (event.key === 'ArrowDown') {
			event.preventDefault();
			selectedIndex = (selectedIndex + 1) % totalItems;
		} else if (event.key === 'ArrowUp') {
			event.preventDefault();
			selectedIndex = (selectedIndex - 1 + totalItems) % totalItems;
		} else if (event.key === 'Enter') {
			event.preventDefault();
			if (hasMathResult && selectedIndex === 0) {
				if (mathResult) writeText(mathResult);
			} else {
				const pluginIndex = selectedIndex - (hasMathResult ? 1 : 0);
				if (filteredPlugins[pluginIndex]) {
					onRunPlugin(filteredPlugins[pluginIndex]);
				}
			}
		}
	}

	function handleItemClick(index: number) {
		selectedIndex = index;
		const pluginIndex = index - (hasMathResult ? 1 : 0);
		onRunPlugin(filteredPlugins[pluginIndex]);
	}

	function numberToWords(numStr: string | null): string {
		if (!numStr) return '';

		const digits: { [key: string]: string } = {
			'0': 'Zero',
			'1': 'One',
			'2': 'Two',
			'3': 'Three',
			'4': 'Four',
			'5': 'Five',
			'6': 'Six',
			'7': 'Seven',
			'8': 'Eight',
			'9': 'Nine',
			'.': 'Point',
			'-': 'Minus'
		};

		const words = Array.from(numStr)
			.map((char) => digits[char] || '')
			.filter(Boolean)
			.join(' ');

		const maxLength = 35;
		if (words.length > maxLength) {
			let truncated = words.substring(0, maxLength);
			const lastSpace = truncated.lastIndexOf(' ');
			if (lastSpace > -1) {
				truncated = truncated.substring(0, lastSpace);
			}
			return truncated + '...';
		}
		return words;
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<main class="bg-background text-foreground flex h-screen flex-col">
	<header class="flex h-12 shrink-0 items-center border-b px-2">
		<Input
			class="rounded-none border-none !bg-transparent pr-0"
			placeholder="Search for extensions and commands..."
			bind:value={searchText}
			autofocus
		/>
	</header>
	<div class="grow overflow-y-auto">
		{#if hasMathResult && mathResult}
			<div class="p-2 pt-2">
				<div class="text-muted-foreground px-2 pb-1 text-xs">Calculator</div>
				<div class="bg-muted flex items-center p-2">
					<div class="flex grow flex-col items-center">
						<span class="truncate text-2xl font-medium">{searchText}</span>
						<span class="bg-input text-muted-foreground truncate rounded px-1.5 py-0.5 text-xs"
							>{inputWords}</span
						>
					</div>
					<div class="grow-0">
						<ArrowRight class="mx-4 size-5 shrink-0 text-gray-400" />
					</div>
					<div class="flex grow flex-col items-center">
						<span class="truncate text-2xl font-semibold">{mathResult}</span>
						<span class="bg-input text-muted-foreground truncate rounded px-1.5 py-0.5 text-xs"
							>{resultWords}</span
						>
					</div>
				</div>
			</div>
		{/if}
		<div>
			{#each filteredPlugins as plugin, index}
				{@const itemIndex = index + (hasMathResult ? 1 : 0)}
				<button
					type="button"
					class="hover:bg-accent/50 flex w-full items-center gap-3 px-4 py-2 text-left"
					class:bg-accent={selectedIndex === itemIndex}
					onclick={() => handleItemClick(itemIndex)}
				>
					<div class="flex size-5 shrink-0 items-center justify-center">
						<Icon icon="app-window-16" class="size-4" />
					</div>
					<div class="flex flex-col">
						<span class="font-medium">{plugin.title}</span>
						<span class="text-muted-foreground text-sm">{plugin.description}</span>
					</div>
					<span class="ml-auto text-xs text-gray-500">{plugin.pluginName}</span>
				</button>
			{/each}
		</div>
	</div>
</main>
