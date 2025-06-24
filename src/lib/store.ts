import * as z from 'zod';

export const AccessSchema = z.enum(['public']);
export type Access = z.infer<typeof AccessSchema>;

export const AvatarPlaceholderColorSchema = z.enum([
	'#A067DC',
	'#BD9500',
	'#D36CDD',
	'#D47600',
	'#D64854',
	'#D85A9B',
	'#DC829A',
	'#EB6B3E',
	'#26B795',
	'#43B93A',
	'#52A9E4',
	'#5385D9',
	'#70920F',
	'#7871E8'
]);
export type AvatarPlaceholderColor = z.infer<typeof AvatarPlaceholderColorSchema>;

export const ModeSchema = z.enum(['menu-bar', 'no-view', 'view']);
export type Mode = z.infer<typeof ModeSchema>;

export const PlatformSchema = z.enum(['macOS', 'Windows']);
export type Platform = z.infer<typeof PlatformSchema>;

export const StatusSchema = z.enum(['active', 'deprecated']);
export type Status = z.infer<typeof StatusSchema>;

export const AuthorSchema = z.object({
	name: z.string(),
	handle: z.string(),
	bio: z.union([z.null(), z.string()]).optional(),
	twitter_handle: z.union([z.null(), z.string()]).optional(),
	github_handle: z.union([z.null(), z.string()]).optional(),
	location: z.union([z.null(), z.string()]).optional(),
	initials: z.string(),
	avatar_placeholder_color: AvatarPlaceholderColorSchema,
	slack_community_username: z.union([z.null(), z.string()]).optional(),
	slack_community_user_id: z.union([z.null(), z.string()]).optional(),
	website_anchor: z.union([z.null(), z.string()]).optional(),
	created_at: z.number().optional(),
	website: z.union([z.null(), z.string()]).optional(),
	username: z.string().optional(),
	avatar: z.union([z.null(), z.string()]),
	id: z.string().optional()
});
export type Author = z.infer<typeof AuthorSchema>;

export const IconsSchema = z.object({
	light: z.union([z.null(), z.string()]),
	dark: z.union([z.null(), z.string()])
});
export type Icons = z.infer<typeof IconsSchema>;

export const ToolSchema = z.object({
	id: z.string(),
	name: z.string(),
	title: z.string(),
	description: z.string(),
	keywords: z.array(z.any()),
	functionalities: z.array(z.any()),
	mode: z.null(),
	side_effects: z.boolean(),
	icons: IconsSchema
});
export type Tool = z.infer<typeof ToolSchema>;

export const CommandSchema = z.object({
	id: z.string(),
	name: z.string(),
	title: z.string(),
	subtitle: z.string(),
	description: z.string(),
	keywords: z.array(z.string()),
	mode: ModeSchema,
	disabled_by_default: z.boolean(),
	beta: z.boolean(),
	icons: IconsSchema
});
export type Command = z.infer<typeof CommandSchema>;

export const DatumSchema = z.object({
	id: z.string(),
	name: z.string(),
	native_id: z.null(),
	seo_categories: z.array(z.string()),
	platforms: z.union([z.array(PlatformSchema), z.null()]),
	created_at: z.number(),
	author: AuthorSchema,
	owner: AuthorSchema,
	status: StatusSchema,
	is_new: z.boolean(),
	access: AccessSchema,
	store_url: z.string(),
	download_count: z.number(),
	kill_listed_at: z.number().nullable(),
	title: z.string(),
	description: z.string(),
	commit_sha: z.string(),
	relative_path: z.string(),
	api_version: z.string(),
	categories: z.array(z.string()),
	prompt_examples: z.array(z.string()),
	metadata_count: z.number(),
	updated_at: z.number(),
	source_url: z.string(),
	readme_url: z.union([z.null(), z.string()]),
	readme_assets_path: z.string(),
	icons: IconsSchema,
	download_url: z.string(),
	commands: z.array(CommandSchema),
	contributors: z.array(AuthorSchema),
	tools: z.array(ToolSchema)
});
export type Datum = z.infer<typeof DatumSchema>;

export const StoreListingsReturnTypeSchema = z.object({
	data: z.array(DatumSchema)
});
export type StoreListingsReturnType = z.infer<typeof StoreListingsReturnTypeSchema>;
