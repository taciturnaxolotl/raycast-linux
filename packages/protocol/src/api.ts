import { z } from 'zod/v4';

export const OpenPayloadSchema = z.object({
	target: z.string(),
	application: z.string().optional()
});
export const OpenMessageSchema = z.object({
	type: z.literal('open'),
	payload: OpenPayloadSchema
});

export const GetSelectedTextPayloadSchema = z.object({
	requestId: z.string()
});
export const GetSelectedTextMessageSchema = z.object({
	type: z.literal('get-selected-text'),
	payload: GetSelectedTextPayloadSchema
});

export const GetSelectedFinderItemsPayloadSchema = z.object({
	requestId: z.string()
});
export const GetSelectedFinderItemsMessageSchema = z.object({
	type: z.literal('get-selected-finder-items'),
	payload: GetSelectedFinderItemsPayloadSchema
});

export const BrowserExtensionRequestPayloadSchema = z.object({
	requestId: z.string(),
	method: z.string(),
	params: z.unknown()
});
export const BrowserExtensionRequestMessageSchema = z.object({
	type: z.literal('browser-extension-request'),
	payload: BrowserExtensionRequestPayloadSchema
});

export const ClipboardContentSchema = z.object({
	text: z.string().optional(),
	html: z.string().optional(),
	file: z.string().optional()
});

export const CopyOptionsSchema = z.object({
	concealed: z.boolean().optional()
});

export const ClipboardCopyPayloadSchema = z.object({
	requestId: z.string(),
	content: ClipboardContentSchema,
	options: CopyOptionsSchema.optional()
});
export const ClipboardCopyMessageSchema = z.object({
	type: z.literal('clipboard-copy'),
	payload: ClipboardCopyPayloadSchema
});

export const ClipboardPastePayloadSchema = z.object({
	requestId: z.string(),
	content: ClipboardContentSchema
});
export const ClipboardPasteMessageSchema = z.object({
	type: z.literal('clipboard-paste'),
	payload: ClipboardPastePayloadSchema
});

export const ClipboardReadPayloadSchema = z.object({
	requestId: z.string(),
	offset: z.number().optional()
});
export const ClipboardReadMessageSchema = z.object({
	type: z.literal('clipboard-read'),
	payload: ClipboardReadPayloadSchema
});

export const ClipboardReadTextPayloadSchema = z.object({
	requestId: z.string(),
	offset: z.number().optional()
});
export const ClipboardReadTextMessageSchema = z.object({
	type: z.literal('clipboard-read-text'),
	payload: ClipboardReadTextPayloadSchema
});

export const ClipboardClearPayloadSchema = z.object({
	requestId: z.string()
});
export const ClipboardClearMessageSchema = z.object({
	type: z.literal('clipboard-clear'),
	payload: ClipboardClearPayloadSchema
});

export const OauthAuthorizePayloadSchema = z.object({
	url: z.string(),
	providerName: z.string(),
	providerIcon: z.string().optional(),
	description: z.string().optional()
});
export const OauthAuthorizeMessageSchema = z.object({
	type: z.literal('oauth-authorize'),
	payload: OauthAuthorizePayloadSchema
});

export const OauthGetTokensPayloadSchema = z.object({
	requestId: z.string(),
	providerId: z.string()
});
export const OauthGetTokensMessageSchema = z.object({
	type: z.literal('oauth-get-tokens'),
	payload: OauthGetTokensPayloadSchema
});

export const OauthSetTokensPayloadSchema = z.object({
	requestId: z.string(),
	providerId: z.string(),
	tokens: z.record(z.string(), z.unknown())
});
export const OauthSetTokensMessageSchema = z.object({
	type: z.literal('oauth-set-tokens'),
	payload: OauthSetTokensPayloadSchema
});

export const OauthRemoveTokensPayloadSchema = z.object({
	requestId: z.string(),
	providerId: z.string()
});
export const OauthRemoveTokensMessageSchema = z.object({
	type: z.literal('oauth-remove-tokens'),
	payload: OauthRemoveTokensPayloadSchema
});

export const SystemGetApplicationsPayloadSchema = z.object({
	requestId: z.string(),
	path: z.string().optional()
});
export const SystemGetApplicationsMessageSchema = z.object({
	type: z.literal('system-get-applications'),
	payload: SystemGetApplicationsPayloadSchema
});

export const SystemGetDefaultApplicationPayloadSchema = z.object({
	requestId: z.string(),
	path: z.string()
});
export const SystemGetDefaultApplicationMessageSchema = z.object({
	type: z.literal('system-get-default-application'),
	payload: SystemGetDefaultApplicationPayloadSchema
});

export const SystemGetFrontmostApplicationPayloadSchema = z.object({
	requestId: z.string()
});
export const SystemGetFrontmostApplicationMessageSchema = z.object({
	type: z.literal('system-get-frontmost-application'),
	payload: SystemGetFrontmostApplicationPayloadSchema
});

export const SystemShowInFinderPayloadSchema = z.object({
	requestId: z.string(),
	path: z.string()
});
export const SystemShowInFinderMessageSchema = z.object({
	type: z.literal('system-show-in-finder'),
	payload: SystemShowInFinderPayloadSchema
});

export const SystemTrashPayloadSchema = z.object({
	requestId: z.string(),
	paths: z.array(z.string())
});
export const SystemTrashMessageSchema = z.object({
	type: z.literal('system-trash'),
	payload: SystemTrashPayloadSchema
});
