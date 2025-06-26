import { z } from 'zod/v4';

export const KeyboardShortcutSchema = z.object({
	modifiers: z.array(z.enum(['cmd', 'ctrl', 'opt', 'shift'])),
	key: z.string()
});
