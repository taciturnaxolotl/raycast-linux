import { _useBaseView, type BaseViewArgs } from './base.svelte';
import type { GridInset, GridSectionProps } from '$lib/props';

export function useGridView(args: () => BaseViewArgs & { columns: number; inset?: GridInset }) {
	const base = _useBaseView(args, 'Grid.Item');
	const { columns, inset: gridInset } = $derived.by(args);

	const processedFlatList = $derived.by(() => {
		const list = base.flatList;
		const newList: ((typeof list)[number] & { inset?: GridInset })[] = [];
		let currentSectionInset: GridInset | undefined;

		for (const item of list) {
			if (item.type === 'header') {
				const sectionProps = item.props as GridSectionProps;
				if (item.id === -1) {
					// This is the synthetic section for top-level items, so it should inherit from the Grid.
					currentSectionInset = gridInset;
				} else {
					// This is a user-defined <Grid.Section>. It does not inherit.
					// If `sectionProps.inset` is undefined, it's treated as "none" by GridItem.
					currentSectionInset = sectionProps.inset;
				}
				newList.push(item);
			} else {
				newList.push({ ...item, inset: currentSectionInset });
			}
		}
		return newList;
	});

	type GridMapItem = {
		flatListIndex: number;
		sectionIndex: number;
		rowIndex: number;
		colIndex: number;
	};
	const gridMap: GridMapItem[] = $derived.by(() => {
		const newGridMap: GridMapItem[] = [];
		let sectionIndex = -1,
			rowIndex = 0,
			colIndex = 0;
		processedFlatList.forEach((item, index) => {
			if (item.type === 'header') {
				sectionIndex++;
				rowIndex = 0;
				colIndex = 0;
			} else if (item.type === 'item') {
				if (colIndex === 0 && newGridMap.length > 0) rowIndex++;
				newGridMap.push({ flatListIndex: index, sectionIndex, rowIndex, colIndex });
				colIndex = (colIndex + 1) % columns;
			}
		});
		return newGridMap;
	});

	$effect(() => {
		if (base.selectedItemIndex < 0) return;
		const elementId = `item-${processedFlatList[base.selectedItemIndex]?.id}`;
		document.getElementById(elementId)?.scrollIntoView({ block: 'nearest' });
	});

	const handleKeydown = (event: KeyboardEvent) => {
		if (!['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(event.key)) return;
		event.preventDefault();

		const currentGridIndex = gridMap.findIndex(
			(item) => item.flatListIndex === base.selectedItemIndex
		);
		if (currentGridIndex === -1) {
			if (gridMap.length > 0) base.selectedItemIndex = gridMap[0].flatListIndex;
			return;
		}

		let newGridIndex = -1;
		const currentPos = gridMap[currentGridIndex];

		if (event.key === 'ArrowLeft') {
			newGridIndex = Math.max(0, currentGridIndex - 1);
		} else if (event.key === 'ArrowRight') {
			newGridIndex = Math.min(gridMap.length - 1, currentGridIndex + 1);
		} else if (event.key === 'ArrowUp' || event.key === 'ArrowDown') {
			const direction = event.key === 'ArrowDown' ? 1 : -1;
			const targetRowIndex = currentPos.rowIndex + direction;
			const itemsInSameSection = gridMap.filter(
				(item) => item.sectionIndex === currentPos.sectionIndex
			);
			let itemsInTargetRow = itemsInSameSection.filter((item) => item.rowIndex === targetRowIndex);

			if (itemsInTargetRow.length === 0) {
				const targetSectionIndex = currentPos.sectionIndex + direction;
				const itemsInTargetSection = gridMap.filter(
					(item) => item.sectionIndex === targetSectionIndex
				);
				if (itemsInTargetSection.length > 0) {
					const rows = [...new Set(itemsInTargetSection.map((i) => i.rowIndex))].sort(
						(a, b) => a - b
					);
					itemsInTargetRow = itemsInTargetSection.filter(
						(i) => i.rowIndex === (direction === 1 ? rows[0] : rows.at(-1))
					);
				}
			}

			if (itemsInTargetRow.length > 0) {
				const targetItem =
					itemsInTargetRow.find((item) => item.colIndex === currentPos.colIndex) ??
					itemsInTargetRow.at(-1)!;
				newGridIndex = gridMap.indexOf(targetItem);
			}
		}

		if (newGridIndex !== -1) {
			base.selectedItemIndex = gridMap[newGridIndex].flatListIndex;
		}
	};

	return {
		get flatList() {
			return processedFlatList;
		},
		get selectedItemIndex() {
			return base.selectedItemIndex;
		},
		setSelectedItemIndex: (index: number) => {
			base.selectedItemIndex = index;
		},
		handleKeydown
	};
}
