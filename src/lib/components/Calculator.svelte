<script lang="ts">
	import { create, all } from 'mathjs';
	import { ArrowRight } from '@lucide/svelte';
	import { writeText } from '@tauri-apps/plugin-clipboard-manager';

	type Props = {
		searchText: string;
		isSelected: boolean;
		onSelect: () => void;
	};

	let { searchText, isSelected, onSelect }: Props = $props();

	const math = create(all);
	let mathResult = $state<string | null>(null);
	let inputWords = $state<string | null>(null);
	let resultWords = $state<string | null>(null);

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

	export function handleClick() {
		onSelect();
		if (mathResult) {
			writeText(mathResult);
		}
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

{#if mathResult}
	<button
		type="button"
		class="w-full p-4 pt-2 text-left"
		class:bg-accent={isSelected}
		onclick={handleClick}
	>
		<div class="text-muted-foreground pb-1 text-xs">Calculator</div>
		{#snippet expression({ value, words }: { value: string | null; words: string | null })}
			<div class="flex grow flex-col items-center gap-2">
				<div class="flex grow items-center truncate text-4xl font-medium">{value}</div>
				<div class="bg-input text-muted-foreground truncate rounded px-2 py-0.5 text-xs">
					{words}
				</div>
			</div>
		{/snippet}
		<div class="bg-muted grid h-40 grid-cols-[1fr_auto_1fr] items-stretch rounded p-4">
			{@render expression({ value: searchText, words: inputWords })}
			<ArrowRight class="text-muted-foreground mx-4 my-auto size-8" />
			{@render expression({ value: mathResult, words: resultWords })}
		</div>
	</button>
{/if}
