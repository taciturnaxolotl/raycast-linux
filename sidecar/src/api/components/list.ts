import { jsx } from 'react/jsx-runtime';
import { createWrapperComponent, createAccessorySlot } from '../utils';

const _AccessorySlot = createAccessorySlot();

const ListPrimitive = createWrapperComponent('List');
const List = (props) => {
	const { searchBarAccessory, children, ...rest } = props;
	const accessoryElement =
		searchBarAccessory &&
		jsx(_AccessorySlot, { name: 'searchBarAccessory', children: searchBarAccessory });
	return jsx(ListPrimitive, { ...rest, children: [children, accessoryElement].filter(Boolean) });
};

const ListItemPrimitive = createWrapperComponent('List.Item');
const ListItem = (props) => {
	const { detail, actions, children, ...rest } = props;
	const detailElement = detail && jsx(_AccessorySlot, { name: 'detail', children: detail });
	const actionsElement = actions && jsx(_AccessorySlot, { name: 'actions', children: actions });
	return jsx(ListItemPrimitive, {
		...rest,
		children: [children, detailElement, actionsElement].filter(Boolean)
	});
};

const ListSection = createWrapperComponent('List.Section');
const ListEmptyView = createWrapperComponent('List.EmptyView');
const ListDropdown = createWrapperComponent('List.Dropdown');
const ListDropdownItem = createWrapperComponent('List.Dropdown.Item');
const ListDropdownSection = createWrapperComponent('List.Dropdown.Section');
const ListItemDetail = createWrapperComponent('List.Item.Detail');
const ListItemDetailMetadata = createWrapperComponent('List.Item.Detail.Metadata');
const ListItemDetailMetadataLabel = createWrapperComponent('List.Item.Detail.Metadata.Label');
const ListItemDetailMetadataLink = createWrapperComponent('List.Item.Detail.Metadata.Link');
const ListItemDetailMetadataTagList = createWrapperComponent('List.Item.Detail.Metadata.TagList');
const ListItemDetailMetadataTagListItem = createWrapperComponent(
	'List.Item.Detail.Metadata.TagList.Item'
);
const ListItemDetailMetadataSeparator = createWrapperComponent(
	'List.Item.Detail.Metadata.Separator'
);

Object.assign(List, {
	Item: ListItem,
	Section: ListSection,
	Dropdown: ListDropdown,
	EmptyView: ListEmptyView
});
Object.assign(ListDropdown, {
	Item: ListDropdownItem,
	Section: ListDropdownSection
});
Object.assign(ListItem, {
	Detail: ListItemDetail
});
Object.assign(ListItemDetail, {
	Metadata: ListItemDetailMetadata
});
Object.assign(ListItemDetailMetadata, {
	Label: ListItemDetailMetadataLabel,
	Link: ListItemDetailMetadataLink,
	TagList: ListItemDetailMetadataTagList,
	Separator: ListItemDetailMetadataSeparator
});
Object.assign(ListItemDetailMetadataTagList, {
	Item: ListItemDetailMetadataTagListItem
});

export { List };
