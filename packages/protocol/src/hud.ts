import { z } from 'zod/v4';

export const ShowHudPayloadSchema = z.object({
	title: z.string()
});

export const ShowHudMessageSchema = z.object({
	type: z.literal('SHOW_HUD'),
	payload: ShowHudPayloadSchema
});
