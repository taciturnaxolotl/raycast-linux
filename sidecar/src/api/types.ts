export const LaunchType = {
	UserInitiated: 'userInitiated',
	Background: 'background'
};

export const Toast = {
	Style: {
		Success: 'SUCCESS',
		Failure: 'FAILURE',
		Animated: 'ANIMATED'
	}
};

export type Application = {
	name: string;
	path: string;
	bundleId?: string;
	localizedName?: string;
};
