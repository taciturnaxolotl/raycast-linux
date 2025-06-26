import { z } from 'zod/v4';
import { PreferenceSchema } from './preferences';

export const PluginInfoSchema = z.object({
	title: z.string(),
	description: z.string().optional(),
	pluginTitle: z.string(),
	pluginName: z.string(),
	commandName: z.string(),
	pluginPath: z.string(),
	icon: z.string().optional(),
	preferences: z.array(PreferenceSchema).optional(),
	commandPreferences: z.array(PreferenceSchema).optional(),
	mode: z.enum(['view', 'no-view', 'menu-bar']).optional()
});
export type PluginInfo = z.infer<typeof PluginInfoSchema>;

export const PluginListSchema = z.object({
	type: z.literal('plugin-list'),
	payload: z.array(PluginInfoSchema)
});
export type PluginList = z.infer<typeof PluginListSchema>;

export const PreferenceValuesSchema = z.object({
	type: z.literal('preference-values'),
	payload: z.object({
		pluginName: z.string(),
		values: z.record(z.string(), z.unknown())
	})
});
export type PreferenceValues = z.infer<typeof PreferenceValuesSchema>;

export const GoBackToPluginListSchema = z.object({
	type: z.literal('go-back-to-plugin-list'),
	payload: z.object({})
});
export type GoBackToPluginList = z.infer<typeof GoBackToPluginListSchema>;
