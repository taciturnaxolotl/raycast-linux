import { z } from 'zod/v4';
import { KeyboardShortcutSchema } from './common';

export const ToastActionOptionsSchema = z.object({
	title: z.string(),
	onAction: z.boolean(),
	shortcut: KeyboardShortcutSchema.optional()
});

export const ToastStyleSchema = z.enum(['SUCCESS', 'FAILURE', 'ANIMATED']);

export const ShowToastPayloadSchema = z.object({
	id: z.number(),
	title: z.string(),
	message: z.string().optional(),
	style: ToastStyleSchema.optional(),
	primaryAction: ToastActionOptionsSchema.optional(),
	secondaryAction: ToastActionOptionsSchema.optional()
});

export const UpdateToastPayloadSchema = z.object({
	id: z.number(),
	title: z.string().optional(),
	message: z.string().optional(),
	style: ToastStyleSchema.optional()
});

export const HideToastPayloadSchema = z.object({
	id: z.number()
});
