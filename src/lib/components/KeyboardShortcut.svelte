<script lang="ts">
	import type { KeyboardShortcut } from '$lib/props';
	import { platform } from '@tauri-apps/plugin-os';
	import { Kbd } from './ui/kbd';

	let { shortcut }: { shortcut: KeyboardShortcut } = $props();

	const macModifierMap = {
		cmd: '⌘',
		ctrl: '⌃',
		opt: '⌥',
		shift: '⇧'
	};

	const standardModifierMap = {
		cmd: 'Ctrl',
		ctrl: 'Ctrl',
		opt: 'Alt',
		shift: 'Shift'
	};

	const modifierMap = platform() === 'macos' ? macModifierMap : standardModifierMap;

	const keyMap: Partial<Record<KeyboardShortcut['key'], string>> = {
		return: '⏎',
		enter: '⏎',
		delete: '⌫',
		backspace: '⌫',
		deleteForward: '⌦',
		arrowUp: '↑',
		arrowDown: '↓',
		arrowLeft: '←',
		arrowRight: '→',
		tab: '⇥',
		escape: '⎋',
		space: '␣'
	};

	const symbols = shortcut.modifiers
		.map((modifier) => modifierMap[modifier])
		.concat(keyMap[shortcut.key] ?? shortcut.key.toUpperCase());
</script>

<div class="flex gap-0.5">
	{#each symbols as symbol}
		<Kbd>{symbol}</Kbd>
	{/each}
</div>
