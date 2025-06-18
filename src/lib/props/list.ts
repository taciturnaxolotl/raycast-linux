import { z } from 'zod/v4';
import { ImageLikeSchema } from './image';
import { DetailMetadataLabelPropsSchema } from './detail';
import { DetailMetadataPropsSchema } from './grid';

export const ListPropsSchema = z.object({
	filtering: z.boolean().optional(),
	isShowingDetail: z.boolean().optional(),
	onSearchTextChange: z.boolean().optional(),
	isLoading: z.boolean().default(false)
});
export type ListProps = z.infer<typeof ListPropsSchema>;

export const ListSectionPropsSchema = z.object({
	title: z.string().optional()
});
export type ListSectionProps = z.infer<typeof ListSectionPropsSchema>;

const ListItemAccessorySchema = z.object({
	text: z.string().optional()
});

export const ListItemPropsSchema = z.object({
	icon: ImageLikeSchema.optional(),
	title: z.string(),
	accessories: z.array(ListItemAccessorySchema).optional(),
	keywords: z.array(z.string()).optional()
});
export type ListItemProps = z.infer<typeof ListItemPropsSchema>;

export const ListItemDetailPropsSchema = z.object({
	isLoading: z.boolean().default(false),
	markdown: z.string().default('')
});
export type ListItemDetailProps = z.infer<typeof ListItemDetailPropsSchema>;

export const ListItemDetailMetadataPropsSchema = DetailMetadataPropsSchema;
export type ListItemDetailMetadataProps = z.infer<typeof ListItemDetailMetadataPropsSchema>;

export const ListDropdownItemPropsSchema = z.object({
	title: z.string(),
	value: z.string(),
	icon: ImageLikeSchema.optional(),
	keywords: z.array(z.string()).optional()
});
export type ListDropdownItemProps = z.infer<typeof ListDropdownItemPropsSchema>;

export const ListDropdownSectionPropsSchema = z.object({
	title: z.string().optional()
});
export type ListDropdownSectionProps = z.infer<typeof ListDropdownSectionPropsSchema>;

export const ListDropdownPropsSchema = z.object({
	tooltip: z.string(),
	defaultValue: z.string().optional(),
	filtering: z.union([z.boolean(), z.object({ keepSectionOrder: z.boolean() })]).optional(),
	id: z.string().optional(),
	isLoading: z.boolean().optional(),
	placeholder: z.string().optional(),
	storeValue: z.boolean().optional(),
	throttle: z.boolean().optional(),
	value: z.string().optional()
});
export type ListDropdownProps = z.infer<typeof ListDropdownPropsSchema>;
