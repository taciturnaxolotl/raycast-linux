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
import ListItemDetail from '$lib/components/nodes/list/ItemDetail.svelte';
import Form from '$lib/components/nodes/form/Form.svelte';
import FormTextArea from '$lib/components/nodes/form/TextArea.svelte';
import FormDescription from '$lib/components/nodes/form/Description.svelte';
import Dropdown from '$lib/components/nodes/shared/Dropdown.svelte';
import DropdownItem from '$lib/components/nodes/shared/DropdownItem.svelte';
import DropdownSection from '$lib/components/nodes/shared/DropdownSection.svelte';

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
	['Grid.Dropdown', Dropdown],
	['Grid.Dropdown.Section', DropdownSection],
	['Grid.Dropdown.Item', DropdownItem],
	['List.Dropdown', Dropdown],
	['List.Dropdown.Item', DropdownItem],
	['List.Dropdown.Section', DropdownSection],
	['List.Item.Detail', ListItemDetail],
	['List.Item.Detail.Metadata', Metadata],
	['List.Item.Detail.Metadata.Label', MetadataLabel],
	['List.Item.Detail.Metadata.Link', MetadataLink],
	['List.Item.Detail.Metadata.TagList', MetadataTagList],
	['List.Item.Detail.Metadata.TagList.Item', MetadataTagListItem],
	['List.Item.Detail.Metadata.Separator', MetadataSeparator],
	['Form', Form],
	['Form.TextArea', FormTextArea],
	['Form.Description', FormDescription],
	['Form.Dropdown', Dropdown],
	['Form.Dropdown.Item', DropdownItem]
]);
