import { getContext } from 'svelte';

type ActionPanelContext = {
	primaryActionNodeId: () => number | undefined;
	secondaryActionNodeId: () => number | undefined;
};

export function useActionRole(nodeId: number) {
	const context: ActionPanelContext | undefined = getContext('ActionPanelContext');
	const isPrimaryAction = $derived(context?.primaryActionNodeId() === nodeId);
	const isSecondaryAction = $derived(context?.secondaryActionNodeId() === nodeId);

	return () => ({ isPrimaryAction, isSecondaryAction });
}
