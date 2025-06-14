import type { KeyboardShortcut } from '$lib/props/actions';

function formatShortcutParts(parts: KeyboardShortcut, isMac: boolean): string {
	const modifierMap = {
		mac: {
			cmd: '⌘',
			ctrl: '⌃',
			opt: '⌥',
			shift: '⇧'
		},
		other: {
			cmd: 'Win',
			ctrl: 'Ctrl',
			opt: 'Alt',
			shift: 'Shift'
		}
	};

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

	const currentModifiers = isMac ? modifierMap.mac : modifierMap.other;

	const modifierStrings = parts.modifiers.map((mod) => currentModifiers[mod]);

	const keyString = keyMap[parts.key] ?? parts.key.toUpperCase();

	const allParts = [...modifierStrings, keyString];

	return allParts.join(' + ');
}

export function shortcutToText(shortcut: KeyboardShortcut, forceOS?: 'macOS' | 'windows'): string {
	const isMac = forceOS
		? forceOS === 'macOS'
		: typeof navigator !== 'undefined' && /Mac/i.test(navigator.platform);

	if ('modifiers' in shortcut) {
		return formatShortcutParts(shortcut, isMac);
	} else {
		return formatShortcutParts(shortcut, true);
	}
}
