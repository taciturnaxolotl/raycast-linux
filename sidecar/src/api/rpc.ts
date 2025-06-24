import { writeOutput } from '../io';
import * as crypto from 'crypto';

const pendingRequests = new Map<
	string,
	{ resolve: (value: unknown) => void; reject: (reason?: unknown) => void }
>();

export function sendRequest<T>(type: string, payload: object = {}): Promise<T> {
	return new Promise((resolve, reject) => {
		const requestId = crypto.randomUUID();
		pendingRequests.set(requestId, { resolve: resolve as (value: unknown) => void, reject });

		writeOutput({
			type,
			payload: { requestId, ...payload }
		});

		setTimeout(() => {
			if (pendingRequests.has(requestId)) {
				pendingRequests.delete(requestId);
				reject(new Error(`Request for ${type} timed out`));
			}
		}, 5000);
	});
}

export function handleResponse(requestId: string, result: unknown, error?: string) {
	const promise = pendingRequests.get(requestId);
	if (promise) {
		if (error) {
			promise.reject(new Error(error));
		} else {
			promise.resolve(result);
		}
		pendingRequests.delete(requestId);
	}
}
