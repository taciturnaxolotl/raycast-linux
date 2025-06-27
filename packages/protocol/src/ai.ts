import { z } from 'zod/v4';

export const AiAskStreamPayloadSchema = z.object({
	requestId: z.string(),
	prompt: z.string(),
	options: z
		.object({
			model: z.string().optional(),
			creativity: z.string().optional(),
			modelMappings: z.record(z.string(), z.string()).optional()
		})
		.optional()
});

export const AiAskStreamMessageSchema = z.object({
	type: z.literal('ai-ask-stream'),
	payload: AiAskStreamPayloadSchema
});

export const AiStreamChunkPayloadSchema = z.object({
	requestId: z.string(),
	text: z.string()
});

export const AiStreamChunkMessageSchema = z.object({
	type: z.literal('ai-stream-chunk'),
	payload: AiStreamChunkPayloadSchema
});

export const AiStreamEndPayloadSchema = z.object({
	requestId: z.string(),
	fullText: z.string()
});

export const AiStreamEndMessageSchema = z.object({
	type: z.literal('ai-stream-end'),
	payload: AiStreamEndPayloadSchema
});

export const AiStreamErrorPayloadSchema = z.object({
	requestId: z.string(),
	error: z.string()
});

export const AiStreamErrorMessageSchema = z.object({
	type: z.literal('ai-stream-error'),
	payload: AiStreamErrorPayloadSchema
});
