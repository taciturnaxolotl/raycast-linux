import { z } from 'zod/v4';
import {
	BrowserExtensionRequestMessageSchema,
	ClipboardClearMessageSchema,
	ClipboardCopyMessageSchema,
	ClipboardPasteMessageSchema,
	ClipboardReadMessageSchema,
	ClipboardReadTextMessageSchema,
	GetSelectedFinderItemsMessageSchema,
	GetSelectedTextMessageSchema,
	OauthAuthorizeMessageSchema,
	OauthGetTokensMessageSchema,
	OauthRemoveTokensMessageSchema,
	OauthSetTokensMessageSchema,
	OpenMessageSchema,
	SystemGetApplicationsMessageSchema,
	SystemGetDefaultApplicationMessageSchema,
	SystemGetFrontmostApplicationMessageSchema,
	SystemShowInFinderMessageSchema,
	SystemTrashMessageSchema
} from './api';
import {
	AiAskStreamMessageSchema,
	AiCanAccessMessageSchema,
	AiStreamChunkMessageSchema,
	AiStreamEndMessageSchema,
	AiStreamErrorMessageSchema
} from './ai';
import { CommandSchema } from './command';
import { ShowHudMessageSchema } from './hud';
import { GoBackToPluginListSchema, PluginListSchema, PreferenceValuesSchema } from './plugin';

export const BatchUpdateSchema = z.object({
	type: z.literal('BATCH_UPDATE'),
	payload: z.array(CommandSchema)
});
export type BatchUpdate = z.infer<typeof BatchUpdateSchema>;

const LogMessageSchema = z.object({
	type: z.literal('log'),
	payload: z.unknown()
});

export const SidecarMessageSchema = z.union([BatchUpdateSchema, CommandSchema, LogMessageSchema]);
export type SidecarMessage = z.infer<typeof SidecarMessageSchema>;

export const SidecarMessageWithPluginsSchema = z.union([
	BatchUpdateSchema,
	CommandSchema,
	ShowHudMessageSchema,
	LogMessageSchema,
	PluginListSchema,
	PreferenceValuesSchema,
	GoBackToPluginListSchema,
	OpenMessageSchema,
	GetSelectedTextMessageSchema,
	GetSelectedFinderItemsMessageSchema,
	BrowserExtensionRequestMessageSchema,
	ClipboardCopyMessageSchema,
	ClipboardPasteMessageSchema,
	ClipboardReadMessageSchema,
	ClipboardReadTextMessageSchema,
	ClipboardClearMessageSchema,
	OauthAuthorizeMessageSchema,
	OauthGetTokensMessageSchema,
	OauthSetTokensMessageSchema,
	OauthRemoveTokensMessageSchema,
	SystemGetApplicationsMessageSchema,
	SystemGetDefaultApplicationMessageSchema,
	SystemGetFrontmostApplicationMessageSchema,
	SystemShowInFinderMessageSchema,
	SystemTrashMessageSchema,
	AiAskStreamMessageSchema,
	AiStreamChunkMessageSchema,
	AiStreamEndMessageSchema,
	AiStreamErrorMessageSchema,
	AiCanAccessMessageSchema
]);
export type SidecarMessageWithPlugins = z.infer<typeof SidecarMessageWithPluginsSchema>;
