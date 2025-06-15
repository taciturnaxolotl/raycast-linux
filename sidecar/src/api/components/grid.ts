import { jsx } from 'react/jsx-runtime';
import { createWrapperComponent, createAccessorySlot } from '../utils';

const _AccessorySlot = createAccessorySlot();

const GridPrimitive = createWrapperComponent('Grid');
const Grid = (props) => {
	const { searchBarAccessory, children, ...rest } = props;
	const accessoryElement =
		searchBarAccessory &&
		jsx(_AccessorySlot, { name: 'searchBarAccessory', children: searchBarAccessory });
	return jsx(GridPrimitive, { ...rest, children: [children, accessoryElement].filter(Boolean) });
};

const GridItemPrimitive = createWrapperComponent('Grid.Item');
const GridItem = (props) => {
	const { detail, actions, children, ...rest } = props;
	const detailElement = detail && jsx(_AccessorySlot, { name: 'detail', children: detail });
	const actionsElement = actions && jsx(_AccessorySlot, { name: 'actions', children: actions });
	return jsx(GridItemPrimitive, {
		...rest,
		children: [children, detailElement, actionsElement].filter(Boolean)
	});
};

const GridSection = createWrapperComponent('Grid.Section');
const GridDropdown = createWrapperComponent('Grid.Dropdown');
const GridDropdownItem = createWrapperComponent('Grid.Dropdown.Item');
const GridDropdownSection = createWrapperComponent('Grid.Dropdown.Section');

Object.assign(Grid, {
	Section: GridSection,
	Item: GridItem,
	Dropdown: GridDropdown
});
Object.assign(GridDropdown, {
	Item: GridDropdownItem,
	Section: GridDropdownSection
});

export { Grid };
