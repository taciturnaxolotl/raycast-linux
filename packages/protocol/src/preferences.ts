import { z } from 'zod/v4';

const BasePreferenceSchema = z.object({
	name: z.string(),
	title: z.string().optional(),
	description: z.string().optional(),
	required: z.boolean().optional(),
	default: z.union([z.string(), z.boolean()]).optional(),
	data: z
		.array(
			z.object({
				title: z.string(),
				value: z.string()
			})
		)
		.optional()
});

export const PreferenceSchema = z.discriminatedUnion('type', [
	BasePreferenceSchema.extend({
		type: z.literal('checkbox'),
		label: z.string()
	}),
	BasePreferenceSchema.extend({
		type: z.enum(['textfield', 'password', 'dropdown', 'appPicker', 'file', 'directory'])
	})
]);
export type Preference = z.infer<typeof PreferenceSchema>;
