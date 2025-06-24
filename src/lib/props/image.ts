import { z } from 'zod/v4';

export const ImageMaskSchema = z.enum(['circle', 'roundedRectangle']);

export const RaycastIconSchema = z.templateLiteral([z.string(), '-16']);

export const ImageLikeSchema = z.union([
	RaycastIconSchema,
	z.string(),
	z.object({
		source: z.union([z.string(), z.object({ light: z.string(), dark: z.string() })]),
		mask: ImageMaskSchema.optional()
	})
]);
export type ImageLike = z.infer<typeof ImageLikeSchema>;
