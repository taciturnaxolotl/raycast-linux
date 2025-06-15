import type { Component } from 'svelte';
import type { UINode } from './types';
import Action from '$lib/components/nodes/actions/Action.svelte';
import ActionPanel from '$lib/components/nodes/ActionPanel.svelte';
import ActionPanelSection from '$lib/components/nodes/ActionPanelSection.svelte';
import ActionCopyToClipboard from '$lib/components/nodes/actions/CopyToClipboard.svelte';
import ActionOpenInBrowser from '$lib/components/nodes/actions/OpenInBrowser.svelte';
import ActionPush from '$lib/components/nodes/actions/ActionPush.svelte';
import Detail from '$lib/components/nodes/detail/Detail.svelte';
import Metadata from '$lib/components/nodes/detail/Metadata.svelte';
import MetadataLabel from '$lib/components/nodes/detail/MetadataLabel.svelte';
import MetadataLink from '$lib/components/nodes/detail/MetadataLink.svelte';
import MetadataTagList from '$lib/components/nodes/detail/MetadataTagList.svelte';
import MetadataTagListItem from '$lib/components/nodes/detail/MetadataTagListItem.svelte';
import MetadataSeparator from '$lib/components/nodes/detail/MetadataSeparator.svelte';
import GridDropdown from '$lib/components/nodes/grid/Dropdown.svelte';
import GridDropdownSection from '$lib/components/nodes/grid/DropdownSection.svelte';
import GridDropdownItem from '$lib/components/nodes/grid/DropdownItem.svelte';
import ListDropdown from '$lib/components/nodes/list/Dropdown.svelte';
import ListDropdownItem from '$lib/components/nodes/list/DropdownItem.svelte';
import ListDropdownSection from '$lib/components/nodes/list/DropdownSection.svelte';
import ListItemDetail from '$lib/components/nodes/list/ItemDetail.svelte';
import ListItemDetailMetadata from '$lib/components/nodes/list/ItemDetailMetadata.svelte';
import ListItemDetailMetadataLabel from '$lib/components/nodes/list/ItemDetailMetadataLabel.svelte';
import ListItemDetailMetadataLink from '$lib/components/nodes/list/ItemDetailMetadataLink.svelte';
import ListItemDetailMetadataTagList from '$lib/components/nodes/list/ItemDetailMetadataTagList.svelte';
import ListItemDetailMetadataTagListItem from '$lib/components/nodes/list/ItemDetailMetadataTagListItem.svelte';
import ListItemDetailMetadataSeparator from '$lib/components/nodes/list/ItemDetailMetadataSeparator.svelte';
import Form from '$lib/components/nodes/form/Form.svelte';
import FormTextArea from '$lib/components/nodes/form/TextArea.svelte';
import FormDescription from '$lib/components/nodes/form/Description.svelte';
import FormDropdown from '$lib/components/nodes/form/Dropdown.svelte';
import FormDropdownItem from '$lib/components/nodes/form/DropdownItem.svelte';

export const componentMap = new Map<
	string,
	Component<{
		nodeId: number;
		uiTree: Map<number, UINode>;
		onDispatch: (instanceId: number, handlerName: string, args: unknown[]) => void;
	}>
>([
	['Action', Action],
	['ActionPanel', ActionPanel],
	['ActionPanel.Section', ActionPanelSection],
	['Action.CopyToClipboard', ActionCopyToClipboard],
	['Action.OpenInBrowser', ActionOpenInBrowser],
	['Action.Push', ActionPush],
	['Detail', Detail],
	['Detail.Metadata', Metadata],
	['Detail.Metadata.Label', MetadataLabel],
	['Detail.Metadata.Link', MetadataLink],
	['Detail.Metadata.TagList', MetadataTagList],
	['Detail.Metadata.TagList.Item', MetadataTagListItem],
	['Detail.Metadata.Separator', MetadataSeparator],
	['Grid.Dropdown', GridDropdown],
	['Grid.Dropdown.Section', GridDropdownSection],
	['Grid.Dropdown.Item', GridDropdownItem],
	['List.Dropdown', ListDropdown],
	['List.Dropdown.Item', ListDropdownItem],
	['List.Dropdown.Section', ListDropdownSection],
	['List.Item.Detail', ListItemDetail],
	['List.Item.Detail.Metadata', ListItemDetailMetadata],
	['List.Item.Detail.Metadata.Label', ListItemDetailMetadataLabel],
	['List.Item.Detail.Metadata.Link', ListItemDetailMetadataLink],
	['List.Item.Detail.Metadata.TagList', ListItemDetailMetadataTagList],
	['List.Item.Detail.Metadata.TagList.Item', ListItemDetailMetadataTagListItem],
	['List.Item.Detail.Metadata.Separator', ListItemDetailMetadataSeparator],
	['Form', Form],
	['Form.TextArea', FormTextArea],
	['Form.Description', FormDescription],
	['Form.Dropdown', FormDropdown],
	['Form.Dropdown.Item', FormDropdownItem]
]);
