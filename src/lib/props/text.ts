import { z } from 'zod/v4';
import { RaycastColorSchema } from './color';

export const TextWithColorSchema = z.object({
	value: z.string(),
	color: RaycastColorSchema
});
export type TextWithColor = z.infer<typeof TextWithColorSchema>;

export const TextLikeSchema = z.union([z.string(), TextWithColorSchema]);
export type TextLike = z.infer<typeof TextLikeSchema>;
