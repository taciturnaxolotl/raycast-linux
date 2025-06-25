<script lang="ts">
	import { fade } from 'svelte/transition';

	type Props = {
		isLoading: boolean;
	};

	let { isLoading }: Props = $props();
</script>

{#if isLoading}
	<div class="loading-indicator" transition:fade={{ duration: 200 }}></div>
{/if}
