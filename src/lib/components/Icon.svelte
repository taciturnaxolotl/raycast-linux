<script lang="ts">
	import type { ImageLike } from '$lib/props';
	import { resolveIcon } from '$lib/assets';
	import icons from '$lib/icons.svg';

	type Props = {
		icon: ImageLike | undefined | null;
		class?: string;
	};
	let { icon, class: className }: Props = $props();

	const iconInfo = $derived(resolveIcon(icon));

	const maskStyles = $derived(
		iconInfo?.type === 'image' && iconInfo.mask === 'Circle' ? 'border-radius: 50%;' : ''
	);
</script>

{#if iconInfo}
	{#if iconInfo.type === 'raycast'}
		<svg class="size-4 shrink-0 fill-none {className}">
			<use href="{icons}#{iconInfo.name}"></use>
		</svg>
	{:else if iconInfo.type === 'image'}
		<img
			src={iconInfo.src}
			alt=""
			class="size-4 object-cover {className ?? ''}"
			style={maskStyles}
		/>
	{:else if iconInfo.type === 'emoji'}
		<span class="size-4 {className ?? ''}">{iconInfo.emoji}</span>
	{/if}
{/if}
