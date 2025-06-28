<script lang="ts">
	import { createToggleGroup, type CreateToggleGroupProps } from '@melt-ui/svelte';
	import { get, writable } from 'svelte/store';

	type $$Props = CreateToggleGroupProps & {
		checked?: boolean;
		onCheckedChange?: (checked: boolean) => void;
	};

	let { checked, onCheckedChange, ...rest }: $$Props = $props();

	const value = writable(checked ? 'on' : 'off');

	$effect(() => {
		if (checked !== undefined && get(value) !== (checked ? 'on' : 'off')) {
			value.set(checked ? 'on' : 'off');
		}
	});

	const { elements } = createToggleGroup({
		...rest,
		value,
		onValueChange: (val) => {
			onCheckedChange?.(val === 'on');
		}
	});

	const { root } = elements;
</script>

<div use:root {...$$restProps} role="switch" aria-checked={checked} data-state={checked ? 'checked' : 'unchecked'}>
	<slot />
</div>
