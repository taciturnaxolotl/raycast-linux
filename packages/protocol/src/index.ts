import { z } from 'zod/v4';

const CreateInstancePayloadSchema = z.object({
	id: z.number(),
	type: z.string(),
	props: z.record(z.string(), z.unknown()),
	children: z.array(z.number()).optional(),
	namedChildren: z.record(z.string(), z.number()).optional()
});

const CreateTextInstancePayloadSchema = z.object({
	id: z.number(),
	type: z.literal('TEXT'),
	text: z.string()
});

const ParentChildPayloadSchema = z.object({
	parentId: z.union([z.literal('root'), z.number()]),
	childId: z.number()
});

const InsertBeforePayloadSchema = z.object({
	parentId: z.union([z.literal('root'), z.number()]),
	childId: z.number(),
	beforeId: z.number()
});

const UpdatePropsPayloadSchema = z.object({
	id: z.number(),
	props: z.record(z.string(), z.unknown()),
	namedChildren: z.record(z.string(), z.number()).optional()
});

const UpdateTextPayloadSchema = z.object({
	id: z.number(),
	text: z.string()
});

const ReplaceChildrenPayloadSchema = z.object({
	parentId: z.union([z.string(), z.number()]),
	childrenIds: z.array(z.number())
});

const ClearContainerPayloadSchema = z.object({
	containerId: z.string()
});

const KeyboardShortcutSchema = z.object({
	modifiers: z.array(z.enum(['cmd', 'ctrl', 'opt', 'shift'])),
	key: z.string()
});

const ToastActionOptionsSchema = z.object({
	title: z.string(),
	onAction: z.boolean(),
	shortcut: KeyboardShortcutSchema.optional()
});

const ToastStyleSchema = z.enum(['SUCCESS', 'FAILURE', 'ANIMATED']);

const ShowToastPayloadSchema = z.object({
	id: z.number(),
	title: z.string(),
	message: z.string().optional(),
	style: ToastStyleSchema.optional(),
	primaryAction: ToastActionOptionsSchema.optional(),
	secondaryAction: ToastActionOptionsSchema.optional()
});

const UpdateToastPayloadSchema = z.object({
	id: z.number(),
	title: z.string().optional(),
	message: z.string().optional(),
	style: ToastStyleSchema.optional()
});

const HideToastPayloadSchema = z.object({
	id: z.number()
});

export const CommandSchema = z.discriminatedUnion('type', [
	z.object({ type: z.literal('CREATE_INSTANCE'), payload: CreateInstancePayloadSchema }),
	z.object({ type: z.literal('CREATE_TEXT_INSTANCE'), payload: CreateTextInstancePayloadSchema }),
	z.object({ type: z.literal('APPEND_CHILD'), payload: ParentChildPayloadSchema }),
	z.object({ type: z.literal('INSERT_BEFORE'), payload: InsertBeforePayloadSchema }),
	z.object({ type: z.literal('REMOVE_CHILD'), payload: ParentChildPayloadSchema }),
	z.object({ type: z.literal('UPDATE_PROPS'), payload: UpdatePropsPayloadSchema }),
	z.object({ type: z.literal('UPDATE_TEXT'), payload: UpdateTextPayloadSchema }),
	z.object({ type: z.literal('REPLACE_CHILDREN'), payload: ReplaceChildrenPayloadSchema }),
	z.object({ type: z.literal('CLEAR_CONTAINER'), payload: ClearContainerPayloadSchema }),
	z.object({ type: z.literal('SHOW_TOAST'), payload: ShowToastPayloadSchema }),
	z.object({ type: z.literal('UPDATE_TOAST'), payload: UpdateToastPayloadSchema }),
	z.object({ type: z.literal('HIDE_TOAST'), payload: HideToastPayloadSchema })
]);
export type Command = z.infer<typeof CommandSchema>;

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

export const PreferenceSchema = z.object({
	name: z.string(),
	title: z.string(),
	description: z.string().optional(),
	type: z.enum(['textfield', 'dropdown', 'checkbox', 'directory']),
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
export type Preference = z.infer<typeof PreferenceSchema>;

export const PluginInfoSchema = z.object({
	title: z.string(),
	description: z.string().optional(),
	pluginName: z.string(),
	commandName: z.string(),
	pluginPath: z.string(),
	icon: z.string().optional(),
	preferences: z.array(PreferenceSchema).optional(),
	mode: z.enum(['view', 'no-view']).optional()
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

const OpenPayloadSchema = z.object({
	target: z.string(),
	application: z.string().optional()
});
const OpenMessageSchema = z.object({
	type: z.literal('open'),
	payload: OpenPayloadSchema
});

const GetSelectedTextPayloadSchema = z.object({
	requestId: z.string()
});
const GetSelectedTextMessageSchema = z.object({
	type: z.literal('get-selected-text'),
	payload: GetSelectedTextPayloadSchema
});

const GetSelectedFinderItemsPayloadSchema = z.object({
	requestId: z.string()
});
const GetSelectedFinderItemsMessageSchema = z.object({
	type: z.literal('get-selected-finder-items'),
	payload: GetSelectedFinderItemsPayloadSchema
});

const BrowserExtensionRequestPayloadSchema = z.object({
	requestId: z.string(),
	method: z.string(),
	params: z.unknown()
});
const BrowserExtensionRequestMessageSchema = z.object({
	type: z.literal('browser-extension-request'),
	payload: BrowserExtensionRequestPayloadSchema
});

export const SidecarMessageWithPluginsSchema = z.union([
	BatchUpdateSchema,
	CommandSchema,
	LogMessageSchema,
	PluginListSchema,
	PreferenceValuesSchema,
	GoBackToPluginListSchema,
	OpenMessageSchema,
	GetSelectedTextMessageSchema,
	GetSelectedFinderItemsMessageSchema,
	BrowserExtensionRequestMessageSchema
]);
export type SidecarMessageWithPlugins = z.infer<typeof SidecarMessageWithPluginsSchema>;
