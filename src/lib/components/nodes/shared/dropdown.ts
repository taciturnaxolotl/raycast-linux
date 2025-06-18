import type { UINode } from '$lib/types';
import { getTypedProps, type ListDropdownItemProps } from '$lib/props';

export function getDropdownItems(
	node: UINode,
	uiTree: Map<number, UINode>
): ListDropdownItemProps[] {
	const items: ListDropdownItemProps[] = [];
	if (!node) return items;

	function traverse(children: number[]) {
		for (const childId of children) {
			const childNode = uiTree.get(childId);
			if (!childNode) continue;

			if (
				childNode.type === 'List.Dropdown.Item' ||
				childNode.type === 'Grid.Dropdown.Item' ||
				childNode.type === 'Form.Dropdown.Item'
			) {
				const itemProps = getTypedProps({
					...childNode,
					type: 'List.Dropdown.Item'
				});
				if (itemProps) {
					items.push(itemProps);
				}
			} else if (
				childNode.type === 'List.Dropdown.Section' ||
				childNode.type === 'Grid.Dropdown.Section' ||
				childNode.type === 'Form.Dropdown.Section'
			) {
				if (childNode.children) traverse(childNode.children);
			}
		}
	}

	if (node.children) traverse(node.children);
	return items;
}
