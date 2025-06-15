import { z } from 'zod/v4';
import { ImageLikeSchema } from './image';

const KeyModifierSchema = z.enum(['cmd', 'ctrl', 'opt', 'shift']);
const KeyEquivalentSchema = z.string();

export const KeyboardShortcutSchema = z.object({
	modifiers: z.array(KeyModifierSchema),
	key: KeyEquivalentSchema
});
export type KeyboardShortcut = z.infer<typeof KeyboardShortcutSchema>;

export const ActionPropsSchema = z.object({
	title: z.string(),
	icon: ImageLikeSchema.optional(),
	shortcut: KeyboardShortcutSchema.optional()
});
export type ActionProps = z.infer<typeof ActionPropsSchema>;

export const ActionPushPropsSchema = z.object({
	title: z.string(),
	icon: ImageLikeSchema.optional(),
	shortcut: KeyboardShortcutSchema.optional()
});

export const ActionCopyToClipboardPropsSchema = z.object({
	content: z.string(),
	title: z.string().optional(),
	icon: ImageLikeSchema.optional(),
	shortcut: KeyboardShortcutSchema.optional()
});
export type ActionCopyToClipboardProps = z.infer<typeof ActionCopyToClipboardPropsSchema>;

export const ActionOpenInBrowserPropsSchema = z.object({
	url: z.url(),
	title: z.string().optional(),
	icon: ImageLikeSchema.optional(),
	shortcut: KeyboardShortcutSchema.optional()
});
export type ActionOpenInBrowserProps = z.infer<typeof ActionOpenInBrowserPropsSchema>;

export const ActionPanelSectionPropsSchema = z.object({
	title: z.string().optional()
});
export type ActionPanelSectionProps = z.infer<typeof ActionPanelSectionPropsSchema>;

export const ActionPanelPropsSchema = z.object({});
export type ActionPanelProps = z.infer<typeof ActionPanelPropsSchema>;
