import { jsx } from 'react/jsx-runtime';
import { createWrapperComponent, createAccessorySlot } from '../utils';

const _AccessorySlot = createAccessorySlot();

const FormPrimitive = createWrapperComponent('Form');
const Form = (props) => {
	const { actions, children, searchBarAccessory, ...rest } = props;
	const accessoryElement =
		searchBarAccessory &&
		jsx(_AccessorySlot, { name: 'searchBarAccessory', children: searchBarAccessory });
	const actionsElement = actions && jsx(_AccessorySlot, { name: 'actions', children: actions });
	return jsx(FormPrimitive, {
		...rest,
		children: [children, accessoryElement, actionsElement].filter(Boolean)
	});
};

const FormTextField = createWrapperComponent('Form.TextField');
const FormTextArea = createWrapperComponent('Form.TextArea');

const FormDropdown = createWrapperComponent('Form.Dropdown');
const FormDropdownItem = createWrapperComponent('Form.Dropdown.Item');
const FormDropdownSection = createWrapperComponent('Form.Dropdown.Section');

Object.assign(FormDropdown, {
	Item: FormDropdownItem,
	Section: FormDropdownSection
});

const FormDescription = createWrapperComponent('Form.Description');

Object.assign(Form, {
	Dropdown: FormDropdown,
	TextField: FormTextField,
	TextArea: FormTextArea,
	Description: FormDescription
});

export { Form };
