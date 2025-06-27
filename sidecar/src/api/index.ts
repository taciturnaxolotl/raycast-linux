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
import {
	environment,
	getSelectedFinderItems,
	getSelectedText,
	open,
	getApplications,
	getDefaultApplication,
	getFrontmostApplication,
	showInFinder,
	trash,
	AI as AIConstant
} from './environment';
import { preferencesStore } from '../preferences';
import { showToast } from './toast';
import { showHUD } from './hud';
import { BrowserExtensionAPI } from './browserExtension';
import { Clipboard } from './clipboard';
import * as OAuth from './oauth';
import { AI } from './ai';
import type { Preference } from '@raycast-linux/protocol';

const Image = {
	Mask: {
		Circle: 'circle',
		RoundedRectangle: 'roundedRectangle'
	}
};

let currentPluginName: string | null = null;
let currentPluginPreferences: Preference[] = [];

export const setCurrentPlugin = (pluginName: string, preferences?: Preference[]) => {
	currentPluginName = pluginName;
	currentPluginPreferences = preferences || [];
};

export const getRaycastApi = () => {
	const LocalStorage = createLocalStorage();

	return {
		LocalStorage,
		Color,
		Cache,
		Icon,
		Image,
		LaunchType,
		Toast,
		OAuth,
		AI: {
			...AI,
			...AIConstant
		},
		Action,
		ActionPanel,
		Detail,
		Form,
		Grid,
		List,
		Clipboard,
		environment,
		getApplications,
		getDefaultApplication,
		getFrontmostApplication,
		getPreferenceValues: () => {
			if (currentPluginName) {
				return preferencesStore.getPreferenceValues(currentPluginName, currentPluginPreferences);
			}
			return {};
		},
		getSelectedFinderItems,
		getSelectedText,
		open,
		showInFinder,
		showToast,
		showHUD,
		trash,
		useNavigation,
		usePersistentState: <T>(
			key: string,
			initialValue: T
		): [T, React.Dispatch<React.SetStateAction<T>>, boolean] => {
			const [state, setState] = React.useState(initialValue);
			return [state, setState, false];
		},
		BrowserExtension: BrowserExtensionAPI
	};
};
