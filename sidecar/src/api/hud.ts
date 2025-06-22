import { writeOutput } from '../io';

export async function showHUD(title: string): Promise<void> {
	writeOutput({
		type: 'SHOW_HUD',
		payload: { title }
	});
}
