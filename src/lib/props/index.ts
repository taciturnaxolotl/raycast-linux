import { z } from 'zod/v4';
import type { UINode } from '../types';
import {
	ActionPropsSchema,
	ActionPushPropsSchema,
	ActionPanelPropsSchema,
	ActionPanelSectionPropsSchema,
	ActionCopyToClipboardPropsSchema,
	ActionOpenInBrowserPropsSchema
} from './actions';
import {
	DetailPropsSchema,
	DetailMetadataLabelPropsSchema,
	DetailMetadataLinkPropsSchema,
	DetailMetadataTagListPropsSchema,
	DetailMetadataTagListItemPropsSchema,
	DetailMetadataSeparatorPropsSchema
} from './detail';
import {
	GridPropsSchema,
	GridSectionPropsSchema,
	GridItemPropsSchema,
	GridDropdownPropsSchema,
	GridDropdownSectionPropsSchema,
	GridDropdownItemPropsSchema,
	DetailMetadataPropsSchema
} from './grid';
import {
	FormPropsSchema,
	FormTextFieldPropsSchema,
	FormTextAreaPropsSchema,
	FormDescriptionPropsSchema,
	FormDropdownPropsSchema,
	FormDropdownItemPropsSchema
} from './form';
import {
	ListPropsSchema,
	ListSectionPropsSchema,
	ListItemPropsSchema,
	ListItemDetailPropsSchema,
	ListItemDetailMetadataPropsSchema,
	ListDropdownPropsSchema,
	ListDropdownSectionPropsSchema,
	ListDropdownItemPropsSchema
} from './list';

export * from './actions';
export * from './detail';
export * from './grid';
export * from './list';
export * from './form';
export * from './image';
export * from './color';

export const componentSchemas = {
	Action: ActionPropsSchema,
	'Action.Push': ActionPushPropsSchema,
	ActionPanel: ActionPanelPropsSchema,
	'ActionPanel.Section': ActionPanelSectionPropsSchema,
	'Action.CopyToClipboard': ActionCopyToClipboardPropsSchema,
	'Action.OpenInBrowser': ActionOpenInBrowserPropsSchema,

	List: ListPropsSchema,
	'List.Section': ListSectionPropsSchema,
	'List.Item': ListItemPropsSchema,
	'List.Dropdown': ListDropdownPropsSchema,
	'List.Dropdown.Section': ListDropdownSectionPropsSchema,
	'List.Dropdown.Item': ListDropdownItemPropsSchema,
	'List.Item.Detail': ListItemDetailPropsSchema,
	'List.Item.Detail.Metadata': ListItemDetailMetadataPropsSchema,
	'List.Item.Detail.Metadata.Label': DetailMetadataLabelPropsSchema,
	'List.Item.Detail.Metadata.Link': DetailMetadataLinkPropsSchema,
	'List.Item.Detail.Metadata.TagList': DetailMetadataTagListPropsSchema,
	'List.Item.Detail.Metadata.TagList.Item': DetailMetadataTagListItemPropsSchema,
	'List.Item.Detail.Metadata.Separator': DetailMetadataSeparatorPropsSchema,

	Grid: GridPropsSchema,
	'Grid.Section': GridSectionPropsSchema,
	'Grid.Item': GridItemPropsSchema,
	'Grid.Dropdown': GridDropdownPropsSchema,
	'Grid.Dropdown.Section': GridDropdownSectionPropsSchema,
	'Grid.Dropdown.Item': GridDropdownItemPropsSchema,

	Form: FormPropsSchema,
	'Form.TextField': FormTextFieldPropsSchema,
	'Form.TextArea': FormTextAreaPropsSchema,
	'Form.Description': FormDescriptionPropsSchema,
	'Form.Dropdown': FormDropdownPropsSchema,
	'Form.Dropdown.Item': FormDropdownItemPropsSchema,

	Detail: DetailPropsSchema,
	'Detail.Metadata': DetailMetadataPropsSchema,
	'Detail.Metadata.Label': DetailMetadataLabelPropsSchema,
	'Detail.Metadata.Link': DetailMetadataLinkPropsSchema,
	'Detail.Metadata.TagList': DetailMetadataTagListPropsSchema,
	'Detail.Metadata.TagList.Item': DetailMetadataTagListItemPropsSchema,
	'Detail.Metadata.Separator': DetailMetadataSeparatorPropsSchema
};

export type Schemas = typeof componentSchemas;
export type ComponentType = keyof Schemas;

export function getTypedProps<T extends ComponentType>(
	node: UINode & { type: T }
): z.infer<Schemas[T]> | null {
	const schema = componentSchemas[node.type];
	const result = schema.safeParse(node.props);
	if (!result.success) {
		console.error(
			`[Props Validation Error] For node ${node.id} (type: ${node.type}):`,
			z.prettifyError(result.error)
		);
		return null;
	}
	return result.data as z.infer<Schemas[T]>;
}
