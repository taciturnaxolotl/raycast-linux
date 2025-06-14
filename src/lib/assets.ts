import { RaycastIconSchema, type ImageLike } from '$lib/props';
import { convertFileSrc } from '@tauri-apps/api/core';

const assetsBasePath = '/home/byte/code/raycast-linux/sidecar/dist/plugin/assets/';

export type ResolvedIcon =
	| { type: 'raycast'; name: string }
	| { type: 'image'; src: string; mask?: string };

export function resolveIcon(icon: ImageLike | undefined | null): ResolvedIcon | null {
	if (!icon) return null;

	if (typeof icon === 'string') {
		if (RaycastIconSchema.safeParse(icon).success) {
			return { type: 'raycast', name: icon };
		}
		return { type: 'image', src: convertFileSrc(assetsBasePath + icon) };
	}

	if (typeof icon === 'object' && 'source' in icon) {
		return {
			type: 'image',
			src: convertFileSrc(assetsBasePath + icon.source),
			mask: icon.mask
		};
	}
	return null;
}
