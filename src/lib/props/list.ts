import { z } from 'zod/v4';
import { ImageLikeSchema } from './image';
import { TextWithColorSchema } from './text';
import { ColorLikeSchema } from './color';

export const ListPropsSchema = z.object({
	filtering: z.boolean().default(true),
	isShowingDetail: z.boolean().optional()
});
export type ListProps = z.infer<typeof ListPropsSchema>;

export const ListSectionPropsSchema = z.object({
	title: z.string().optional()
});
export type ListSectionProps = z.infer<typeof ListSectionPropsSchema>;

const ListItemAccessorySchema = z.object({
	text: z.string()
});

export const ListItemPropsSchema = z.object({
	icon: ImageLikeSchema.optional(),
	title: z.string(),
	accessories: z.array(ListItemAccessorySchema).optional(),
	keywords: z.array(z.string()).optional()
});
export type ListItemProps = z.infer<typeof ListItemPropsSchema>;

// TODO: List.Item.Detail is very similar to just Detail; however, there's some minor differences
// should we merge them? they're technically different APIs
export const ListItemDetailPropsSchema = z.object({
	isLoading: z.boolean().default(false),
	markdown: z.string().default('')
});
export type ListItemDetailProps = z.infer<typeof ListItemDetailPropsSchema>;

export const ListItemDetailMetadataPropsSchema = z.object({});
export type ListItemDetailMetadataProps = z.infer<typeof ListItemDetailMetadataPropsSchema>;

export const ListItemDetailMetadataLabelPropsSchema = z.object({
	title: z.string(),
	icon: ImageLikeSchema.optional(),
	text: z.union([z.string(), TextWithColorSchema]).optional()
});
export type ListItemDetailMetadataLabelProps = z.infer<
	typeof ListItemDetailMetadataLabelPropsSchema
>;

export const ListItemDetailMetadataLinkPropsSchema = z.object({
	target: z.string(),
	text: z.string(),
	title: z.string()
});
export type ListItemDetailMetadataLinkProps = z.infer<typeof ListItemDetailMetadataLinkPropsSchema>;

export const ListItemDetailMetadataTagListPropsSchema = z.object({
	title: z.string()
});
export type ListItemDetailMetadataTagListProps = z.infer<
	typeof ListItemDetailMetadataTagListPropsSchema
>;

export const ListItemDetailMetadataTagListItemPropsSchema = z
	.object({
		color: ColorLikeSchema.optional(),
		icon: ImageLikeSchema.optional(),
		text: z.string().optional()
	})
	.refine((data) => data.text != null || data.icon != null, {
		error: 'Either text or icon must be provided for TagList.Item'
	});
export type ListItemDetailMetadataTagListItemProps = z.infer<
	typeof ListItemDetailMetadataTagListItemPropsSchema
>;

export const ListItemDetailMetadataSeparatorPropsSchema = z.object({});
export type ListItemDetailMetadataSeparatorProps = z.infer<
	typeof ListItemDetailMetadataSeparatorPropsSchema
>;
