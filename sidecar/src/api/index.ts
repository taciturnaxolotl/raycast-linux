import React from 'react';
import { Color } from './colors';
import { Cache } from './cache';
import { Icon } from './icon';
import { LaunchType, Toast } from './types';
import { createLocalStorage } from './utils';
import { useNavigation } from './navigation';
import { List } from './components/list';
import { Grid } from './components/grid';
import { Form } from './components/form';
import { Action, ActionPanel } from './components/actions';
import { Detail } from './components/detail';
import { environment, getSelectedFinderItems, getSelectedText } from './environment';
import { preferencesStore } from '../preferences';

let currentPluginName: string | null = null;
let currentPluginPreferences: Array<{
	name: string;
	title: string;
	description?: string;
	type: 'textfield' | 'dropdown' | 'checkbox' | 'directory';
	required?: boolean;
	default?: string | boolean;
	data?: Array<{ title: string; value: string }>;
}> = [];

export const setCurrentPlugin = (
	pluginName: string,
	preferences?: typeof currentPluginPreferences
) => {
	currentPluginName = pluginName;
	currentPluginPreferences = preferences || [];
};

export const getRaycastApi = () => {
	const LocalStorage = createLocalStorage();

	return {
		LocalStorage,
		Color,
		Cache,
		LaunchType,
		getSelectedFinderItems,
		getSelectedText,
		showToast: () => {},
		Toast,
		environment,
		getPreferenceValues: () => {
			if (currentPluginName) {
				return preferencesStore.getPreferenceValues(currentPluginName, currentPluginPreferences);
			}
			return {
				lang1: 'en',
				lang2: 'zh-CN',
				autoInput: true,
				defaultAction: 'copy'
			};
		},
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
		Form,
		Icon
	};
};
