<script lang="ts">
	import { tv } from 'tailwind-variants';

	const switchThumbVariants = tv({
		base: 'pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0',
		variants: {},
		defaultVariants: {}
	});

	type $$Props = {
		class?: string;
	};

	let { class: className }: $$Props = $props();
</script>

<span class={switchThumbVariants({ class: className })} {...$$restProps} />
