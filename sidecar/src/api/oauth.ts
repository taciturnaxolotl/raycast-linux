import * as crypto from 'crypto';
import { writeOutput, writeLog } from '../io';

export enum RedirectMethod {
	Web = 'web',
	App = 'app',
	AppURI = 'app-uri'
}

export interface PKCEClientOptions {
	redirectMethod: RedirectMethod;
	providerName: string;
	providerIcon?: string;
	description?: string;
	providerId?: string;
}

export interface AuthorizationRequestOptions {
	endpoint: string;
	clientId: string;
	scope: string;
	extraParameters?: { [key: string]: string };
}

export interface AuthorizationRequest {
	url: string;
	codeVerifier: string;
	codeChallenge: string;
	redirectURI: string;
	state: string;
	toURL: () => string;
}

export interface AuthorizationOptions {
	url: string;
}

export interface AuthorizationResponse {
	authorizationCode: string;
}

export interface TokenResponse {
	access_token: string;
	refresh_token?: string;
	expires_in?: number;
	scope?: string;
	id_token?: string;
}

export interface TokenSetOptions {
	accessToken: string;
	refreshToken?: string;
	expiresIn?: number;
	scope?: string;
	idToken?: string;
}

export interface TokenSet {
	accessToken: string;
	refreshToken?: string;
	expiresIn?: number;
	scope?: string;
	idToken?: string;
	updatedAt: Date;
	isExpired: () => boolean;
}

const pendingAuthorizationRequests = new Map<
	string,
	{ resolve: (value: AuthorizationResponse) => void; reject: (reason?: unknown) => void }
>();

const pendingTokenRequests = new Map<
	string,
	{ resolve: (value: unknown) => void; reject: (reason?: unknown) => void }
>();

export function handleOAuthResponse(
	_requestId: string,
	code: string,
	state: string,
	error?: string
) {
	const promise = pendingAuthorizationRequests.get(state);
	if (promise) {
		if (error) {
			promise.reject(new Error(error));
		} else {
			promise.resolve({ authorizationCode: code });
		}
		pendingAuthorizationRequests.delete(state);
	} else {
		writeLog(`OAuth state mismatch. Request ID (state): ${state} not found in pending requests.`);
	}
}

export function handleTokenResponse(requestId: string, result: unknown, error?: string) {
	const promise = pendingTokenRequests.get(requestId);
	if (promise) {
		if (error) {
			promise.reject(new Error(error));
		} else {
			promise.resolve(result);
		}
		pendingTokenRequests.delete(requestId);
	}
}

function sendTokenRequest<T>(type: string, payload: object): Promise<T> {
	return new Promise((resolve, reject) => {
		const requestId = crypto.randomUUID();
		pendingTokenRequests.set(requestId, { resolve: resolve as (value: unknown) => void, reject });
		writeOutput({
			type,
			payload: { requestId, ...payload }
		});
		setTimeout(() => {
			if (pendingTokenRequests.has(requestId)) {
				pendingTokenRequests.delete(requestId);
				reject(new Error(`Token request for ${type} timed out`));
			}
		}, 5000);
	});
}

interface StoredTokenSet extends TokenSetOptions {
	updatedAt: string;
}

export class PKCEClient {
	private options: PKCEClientOptions;

	constructor(options: PKCEClientOptions) {
		this.options = options;
	}

	private getProviderId(): string {
		return this.options.providerId ?? this.options.providerName.toLowerCase().replace(/\s/g, '-');
	}

	async authorizationRequest(options: AuthorizationRequestOptions): Promise<AuthorizationRequest> {
		const codeVerifier = crypto.randomBytes(32).toString('base64url');
		const codeChallenge = crypto.createHash('sha256').update(codeVerifier).digest('base64url');
		const state = JSON.stringify({
			providerName: this.options.providerName,
			id: crypto.randomUUID(),
			flavor: 'release'
		});

		let redirectURI: string;
		const packageName = 'Extension'; // TODO: what does this mean, and is it always the same?
		switch (this.options.redirectMethod) {
			case RedirectMethod.Web:
				redirectURI = `https://raycast.com/redirect?packageName=${packageName}`;
				break;
			case RedirectMethod.App:
				redirectURI = `raycast://oauth?package_name=${packageName}`;
				break;
			case RedirectMethod.AppURI:
				redirectURI = `com.raycast:/oauth?package_name=${packageName}`;
				break;
		}

		const urlParams = new URLSearchParams({
			response_type: 'code',
			client_id: options.clientId,
			scope: options.scope,
			redirect_uri: redirectURI,
			state: state,
			code_challenge: codeChallenge,
			code_challenge_method: 'S256',
			...options.extraParameters
		});

		const authRequest: AuthorizationRequest = {
			url: `${options.endpoint}?${urlParams.toString()}`,
			codeVerifier,
			codeChallenge,
			redirectURI,
			state,
			toURL: () => authRequest.url
		};

		return authRequest;
	}

	async authorize(
		authRequest: AuthorizationRequest | AuthorizationOptions
	): Promise<AuthorizationResponse> {
		const state =
			'state' in authRequest
				? authRequest.state
				: new URL(authRequest.url).searchParams.get('state');

		if (!state) {
			throw new Error('State parameter is missing from authorization request.');
		}

		return new Promise((resolve, reject) => {
			pendingAuthorizationRequests.set(state, { resolve, reject });

			writeOutput({
				type: 'oauth-authorize',
				payload: {
					url: authRequest.url,
					providerName: this.options.providerName,
					providerIcon: this.options.providerIcon,
					description: this.options.description
				}
			});

			setTimeout(
				() => {
					if (pendingAuthorizationRequests.has(state)) {
						pendingAuthorizationRequests.delete(state);
						reject(new Error('OAuth authorization timed out'));
					}
				},
				5 * 60 * 1000
			);
		});
	}

	async getTokens(): Promise<TokenSet | undefined> {
		const tokenData = await sendTokenRequest<StoredTokenSet | undefined>('oauth-get-tokens', {
			providerId: this.getProviderId()
		});

		if (!tokenData) {
			return undefined;
		}

		const updatedAt = new Date(tokenData.updatedAt);
		const expiresIn = tokenData.expiresIn;

		const tokenSet: TokenSet = {
			...tokenData,
			updatedAt: updatedAt,
			isExpired: () => {
				if (!expiresIn) {
					return false;
				}
				const now = new Date();
				const expiryDate = new Date(updatedAt.getTime() + expiresIn * 1000);
				return now.getTime() > expiryDate.getTime() - 60000;
			}
		};

		return tokenSet;
	}

	async setTokens(tokens: TokenSetOptions | TokenResponse): Promise<void> {
		let tokenSetOptions: TokenSetOptions;

		if ('access_token' in tokens) {
			tokenSetOptions = {
				accessToken: tokens.access_token,
				refreshToken: tokens.refresh_token,
				expiresIn: tokens.expires_in,
				scope: tokens.scope,
				idToken: tokens.id_token
			};
		} else {
			tokenSetOptions = tokens;
		}

		const payload = {
			...tokenSetOptions,
			updatedAt: new Date().toISOString()
		};

		await sendTokenRequest<void>('oauth-set-tokens', {
			providerId: this.getProviderId(),
			tokens: payload
		});
	}

	async removeTokens(): Promise<void> {
		await sendTokenRequest<void>('oauth-remove-tokens', {
			providerId: this.getProviderId()
		});
	}
}
