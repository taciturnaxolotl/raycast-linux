import type * as api from '@raycast/api';
import { writeOutput } from '../io';
import { getNextInstanceId, toasts } from '../state';
import { Toast as ToastEnum } from './types';

class ToastImpl implements api.Toast {
	#id: number;
	#style: api.Toast.Style;
	#title: string;
	#message?: string;
	primaryAction?: api.Toast.ActionOptions;
	secondaryAction?: api.Toast.ActionOptions;

	constructor(options: api.Toast.Options) {
		this.#id = getNextInstanceId();
		this.#style = options.style ?? ToastEnum.Style.Success;
		this.#title = options.title;
		this.#message = options.message;
		this.primaryAction = options.primaryAction;
		this.secondaryAction = options.secondaryAction;
	}

	get id() {
		return this.#id;
	}

	get style() {
		return this.#style;
	}
	set style(newStyle: api.Toast.Style) {
		this.#style = newStyle;
		this._update();
	}

	get title() {
		return this.#title;
	}
	set title(newTitle: string) {
		this.#title = newTitle;
		this._update();
	}

	get message() {
		return this.#message;
	}
	set message(newMessage: string | undefined) {
		this.#message = newMessage;
		this._update();
	}

	private _update() {
		writeOutput({
			type: 'UPDATE_TOAST',
			payload: {
				id: this.#id,
				style: this.#style,
				title: this.#title,
				message: this.#message
			}
		});
	}

	async hide(): Promise<void> {
		writeOutput({ type: 'HIDE_TOAST', payload: { id: this.#id } });
		toasts.delete(this.#id);
	}

	async show(): Promise<void> {
		this._sendShowCommand();
	}

	_sendShowCommand() {
		toasts.set(this.id, this);
		writeOutput({
			type: 'SHOW_TOAST',
			payload: {
				id: this.#id,
				style: this.style,
				title: this.title,
				message: this.message,
				primaryAction: this.primaryAction
					? {
							title: this.primaryAction.title,
							onAction: !!this.primaryAction.onAction,
							shortcut: this.primaryAction.shortcut
						}
					: undefined,
				secondaryAction: this.secondaryAction
					? {
							title: this.secondaryAction.title,
							onAction: !!this.secondaryAction.onAction,
							shortcut: this.secondaryAction.shortcut
						}
					: undefined
			}
		});
	}
}

export async function showToast(options: api.Toast.Options): Promise<api.Toast> {
	const toast = new ToastImpl(options);
	toast._sendShowCommand();
	return toast;
}
