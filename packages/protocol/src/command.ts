import { z } from 'zod/v4';
import { ShowToastPayloadSchema, UpdateToastPayloadSchema, HideToastPayloadSchema } from './toast';

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
