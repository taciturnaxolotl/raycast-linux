import React from 'react';
import { jsx } from 'react/jsx-runtime';
import { createWrapperComponent } from '../utils';
import { currentRootElement, navigationStack } from '../../state';
import { updateContainer } from '../../reconciler';

const Action = createWrapperComponent('Action');
const ActionPanel = createWrapperComponent('ActionPanel');
const ActionPanelSection = createWrapperComponent('ActionPanel.Section');
const ActionPanelSubmenu = createWrapperComponent('ActionPanel.Submenu');
const ActionPaste = createWrapperComponent('Action.Paste');
const ActionCopy = createWrapperComponent('Action.CopyToClipboard');
const ActionOpenInBrowser = createWrapperComponent('Action.OpenInBrowser');

const ActionPush = ({
	onPush,
	target,
	...props
}: {
	onPush: () => void;
	target: React.ReactElement;
}) => {
	const handleAction = () => {
		if (currentRootElement) {
			navigationStack.push(currentRootElement);
		}
		updateContainer(target);
		onPush?.();
	};
	return jsx('Action.Push', { ...props, onAction: handleAction });
};

Object.assign(Action, {
	Paste: ActionPaste,
	CopyToClipboard: ActionCopy,
	OpenInBrowser: ActionOpenInBrowser,
	Push: ActionPush
});
Object.assign(ActionPanel, {
	Section: ActionPanelSection,
	Submenu: ActionPanelSubmenu
});

export { Action, ActionPanel };
