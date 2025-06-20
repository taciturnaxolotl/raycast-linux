import { RaycastIconSchema, type ImageLike } from '$lib/props';
import { convertFileSrc } from '@tauri-apps/api/core';
import { mode } from 'mode-watcher';
import path from 'path';

// this matches any emoji character (u flag = unicode, \p{Emoji} = any unicode emoji)
const EMOJI_REGEX = /\p{Emoji}/u;
const graphemeSegmenter = new Intl.Segmenter();

const iconIsEmoji = (icon: string) => {
	// explanation: some emojis are made up of multiple characters, so we need to check if the icon is a single emoji
	// first of all, we need to segment the icon into graphemes (characters)
	// if it has a single visual character, and one of the corresponding code points is an emoji, then we consider it to be an emoji
	// i genuinely hate unicode pls send help
	const graphemes = graphemeSegmenter.segment(icon);

	return Array.from(graphemes).length === 1 && EMOJI_REGEX.test(icon);
};

export type ResolvedIcon =
	| { type: 'raycast'; name: string }
	| { type: 'image'; src: string; mask?: string }
	| { type: 'emoji'; emoji: string };

export function resolveIcon(
	icon: ImageLike | undefined | null,
	assetsBasePath: string
): ResolvedIcon | null {
	if (!icon) return null;

	if (typeof icon === 'string') {
		if (iconIsEmoji(icon)) {
			return { type: 'emoji', emoji: icon };
		}

		if (RaycastIconSchema.safeParse(icon).success) {
			return { type: 'raycast', name: icon };
		}

		return { type: 'image', src: convertFileSrc(path.join(assetsBasePath, icon)) };
	}

	if (typeof icon === 'object' && 'source' in icon) {
		if (typeof icon.source === 'object') {
			return {
				type: 'image',
				src: mode.current === 'dark' ? icon.source.dark : icon.source.light,
				mask: icon.mask
			};
		}
		if (icon.source.startsWith('http')) {
			return {
				type: 'image',
				src: icon.source,
				mask: icon.mask
			};
		}
		return {
			type: 'image',
			src: convertFileSrc(path.join(assetsBasePath, icon.source)),
			mask: icon.mask
		};
	}

	return null;
}
