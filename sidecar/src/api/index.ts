import React from 'react';
import { Color } from './colors';
import { Cache } from './cache';
import { Icon } from './icon';
import { LaunchType, Toast } from './types';
import { createLocalStorage } from './utils';
import { useNavigation } from './navigation';
import { List } from './components/list';
import { Grid } from './components/grid';
import { Action, ActionPanel } from './components/actions';
import { Detail } from './components/detail';

export const getRaycastApi = () => {
	const LocalStorage = createLocalStorage();

	return {
		LocalStorage,
		Color,
		Cache,
		LaunchType,
		showToast: () => {},
		Toast,
		environment: {
			assetsPath: '/home/byte/code/raycast-linux/sidecar/dist/plugin/assets/',
			launchType: LaunchType.UserInitiated
		},
		getPreferenceValues: () => ({
			lang1: 'en',
			lang2: 'zh-CN',
			autoInput: true,
			defaultAction: 'copy'
		}),
		usePersistentState: <T>(
			key: string,
			initialValue: T
		): [T, React.Dispatch<React.SetStateAction<T>>, boolean] => {
			const [state, setState] = React.useState(initialValue);
			return [state, setState, false];
		},
		useNavigation,
		List,
		Grid,
		Action,
		ActionPanel,
		Detail,
		Icon
	};
};
