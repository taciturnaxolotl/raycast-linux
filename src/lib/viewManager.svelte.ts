import type { PluginInfo } from '@raycast-linux/protocol';
import { uiStore } from '$lib/ui.svelte';
import { sidecarService } from '$lib/sidecar.svelte';
import type { Quicklink } from './quicklinks.svelte';

export type ViewState =
	| 'command-palette'
	| 'plugin-running'
	| 'settings'
	| 'extensions-store'
	| 'clipboard-history'
	| 'search-snippets'
	| 'quicklink-form'
	| 'create-snippet-form'
	| 'import-snippets';

type OauthState = {
	url: string;
	providerName: string;
	providerIcon?: string;
	description?: string;
} | null;

class ViewManager {
	currentView = $state<ViewState>('command-palette');
	quicklinkToEdit = $state<Quicklink | undefined>(undefined);
	snippetsForImport = $state<any[] | null>(null);

	oauthState: OauthState = $state(null);
	oauthStatus: 'initial' | 'authorizing' | 'success' | 'error' = $state('initial');

	showCommandPalette = () => {
		this.currentView = 'command-palette';
		uiStore.setCurrentRunningPlugin(null);
		this.snippetsForImport = null;
	};

	showSettings = () => {
		this.currentView = 'settings';
	};

	showExtensions = () => {
		this.currentView = 'extensions-store';
	};

	showClipboardHistory = () => {
		this.currentView = 'clipboard-history';
	};

	showSearchSnippets = () => {
		this.currentView = 'search-snippets';
	};

	showQuicklinkForm = (quicklink?: Quicklink) => {
		this.quicklinkToEdit = quicklink;
		this.currentView = 'quicklink-form';
	};

	showCreateSnippetForm = () => {
		this.currentView = 'create-snippet-form';
	};

	showImportSnippets = (snippets?: any[]) => {
		this.snippetsForImport = snippets ?? null;
		this.currentView = 'import-snippets';
	};

	runPlugin = (plugin: PluginInfo) => {
		switch (plugin.pluginPath) {
			case 'builtin:store':
				this.showExtensions();
				return;
			case 'builtin:history':
				this.showClipboardHistory();
				return;
			case 'builtin:search-snippets':
				this.showSearchSnippets();
				return;
			case 'builtin:create-quicklink':
				this.showQuicklinkForm();
				return;
			case 'builtin:create-snippet':
				this.showCreateSnippetForm();
				return;
			case 'builtin:import-snippets':
				this.showImportSnippets();
				return;
		}

		uiStore.setCurrentRunningPlugin(plugin);
		sidecarService.dispatchEvent('run-plugin', {
			pluginPath: plugin.pluginPath,
			commandName: plugin.commandName,
			mode: plugin.mode
		});

		if (plugin.mode !== 'no-view') {
			uiStore.resetForNewPlugin();
			this.currentView = 'plugin-running';
		}
	};

	handleOauthSignIn = () => {
		if (this.oauthState?.url) {
			this.oauthStatus = 'authorizing';
		}
	};

	handleDeepLink = (url: string) => {
		try {
			const urlObj = new URL(url);

			if (urlObj.protocol === 'raycast:') {
				if (urlObj.host === 'snippets' && urlObj.pathname === '/import') {
					const snippetParams = urlObj.searchParams.getAll('snippet');
					const snippets = snippetParams
						.map((param) => {
							try {
								return JSON.parse(decodeURIComponent(param));
							} catch (e) {
								console.error('Failed to parse snippet JSON:', e);
								return null;
							}
						})
						.filter((s): s is object => s !== null);

					this.showImportSnippets(snippets.length > 0 ? snippets : undefined);
				} else if (urlObj.host === 'oauth' || urlObj.pathname.startsWith('/redirect')) {
					const params = urlObj.searchParams;
					const code = params.get('code');
					const state = params.get('state');

					if (this.oauthState) {
						this.oauthStatus = 'success';
						setTimeout(() => {
							this.oauthState = null;
							this.oauthStatus = 'initial';
						}, 2000);
					}

					if (code && state) {
						sidecarService.dispatchEvent('oauth-authorize-response', { code, state });
					} else {
						const error = params.get('error') || 'Unknown OAuth error';
						const errorDescription = params.get('error_description');
						sidecarService.dispatchEvent('oauth-authorize-response', {
							state,
							error: `${error}: ${errorDescription}`
						});
					}
				} else {
					switch (urlObj.host) {
						case 'extensions':
							this.showExtensions();
							break;
						default:
							this.showCommandPalette();
					}
				}
			}
		} catch (error) {
			console.error('Error parsing deep link:', error);
			this.showCommandPalette();
		}
	};
}

export const viewManager = new ViewManager();
