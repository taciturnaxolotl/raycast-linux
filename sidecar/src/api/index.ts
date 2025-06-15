import React from 'react';
import { jsx } from 'react/jsx-runtime';
import { Color } from './colors';
import { Cache } from './cache';
import { Icon } from './icon';
import { currentRootElement, navigationStack } from '../state';
import { updateContainer } from '../reconciler';

export const getRaycastApi = () => {
	const storage = new Map<string, string>();
	const LocalStorage = {
		getItem: async (key: string) => storage.get(key),
		setItem: async (key: string, value: string) => storage.set(key, value),
		removeItem: async (key: string) => storage.delete(key),
		clear: async () => storage.clear()
	};

	const createWrapperComponent = (name: string) => {
		const Component = ({ children, ...rest }: { children?: React.ReactNode }) =>
			jsx(name as any, { ...rest, children });
		Component.displayName = name;
		return Component;
	};

	const _AccessorySlot = createWrapperComponent('_AccessorySlot');

	const useNavigation = () => {
		const push = (element: React.ReactElement) => {
			if (currentRootElement) {
				navigationStack.push(currentRootElement);
				updateContainer(element);
			}
		};

		const pop = () => {
			const previous = navigationStack.pop();
			if (previous) {
				updateContainer(previous);
			}
		};

		return { push, pop };
	};

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

	const Action = createWrapperComponent('Action');
	const ActionPanel = createWrapperComponent('Action.Panel');
	const ActionPanelSection = createWrapperComponent('Action.Panel.Section');
	const ActionPaste = createWrapperComponent('Action.Paste');
	const ActionCopy = createWrapperComponent('Action.CopyToClipboard');
	const ActionOpenInBrowser = createWrapperComponent('Action.OpenInBrowser');
	const ActionPush = ({
		onPush,
		target,
		...props
	}: {
		onPush: () => void;
		target: React.ReactElement;
	}) => {
		const handleAction = () => {
			if (currentRootElement) {
				navigationStack.push(currentRootElement);
			}
			updateContainer(target);
			onPush?.();
		};
		return jsx('Action.Push', { ...props, onAction: handleAction });
	};
	Object.assign(Action, {
		Paste: ActionPaste,
		CopyToClipboard: ActionCopy,
		OpenInBrowser: ActionOpenInBrowser,
		Push: ActionPush
	});
	Object.assign(ActionPanel, {
		Section: ActionPanelSection
	});

	const DetailPrimitive = createWrapperComponent('Detail');
	const Detail = (props) => {
		const { metadata, actions, children, ...rest } = props;
		const metadataElement =
			metadata && jsx(_AccessorySlot, { name: 'metadata', children: metadata });
		const actionsElement = actions && jsx(_AccessorySlot, { name: 'actions', children: actions });
		return jsx(DetailPrimitive, {
			...rest,
			children: [children, metadataElement, actionsElement].filter(Boolean)
		});
	};

	const DetailMetadata = createWrapperComponent('Detail.Metadata');
	const DetailMetadataLabel = createWrapperComponent('Detail.Metadata.Label');
	const DetailMetadataLink = createWrapperComponent('Detail.Metadata.Link');
	const DetailMetadataTagList = createWrapperComponent('Detail.Metadata.TagList');
	const DetailMetadataTagListItem = createWrapperComponent('Detail.Metadata.TagList.Item');
	const DetailMetadataSeparator = createWrapperComponent('Detail.Metadata.Separator');
	Object.assign(Detail, {
		Metadata: DetailMetadata
	});
	Object.assign(DetailMetadata, {
		Label: DetailMetadataLabel,
		Link: DetailMetadataLink,
		TagList: DetailMetadataTagList,
		Separator: DetailMetadataSeparator
	});
	Object.assign(DetailMetadataTagList, {
		Item: DetailMetadataTagListItem
	});

	const LaunchType = {
		UserInitiated: 'userInitiated',
		Background: 'background'
	};

	return {
		LocalStorage,
		Color,
		Cache,
		LaunchType,
		showToast: () => {},
		Toast: {
			Style: {
				Success: 'SUCCESS',
				Failure: 'FAILURE',
				Animated: 'ANIMATED'
			}
		},
		environment: {
			assetsPath: '/home/byte/code/raycast-linux/sidecar/dist/plugin/assets/',
			launchType: LaunchType.UserInitiated
		},
		getPreferenceValues: () => ({
			language: 9
		}),
		usePersistentState: <T>(
			key: string,
			initialValue: T
		): [T, React.Dispatch<React.SetStateAction<T>>, boolean] => {
			const [state, setState] = React.useState(initialValue);
			return [state, setState, false];
		},
		useNavigation,
		List,
		Grid,
		Action,
		ActionPanel,
		Detail,
		Icon
	};
};
