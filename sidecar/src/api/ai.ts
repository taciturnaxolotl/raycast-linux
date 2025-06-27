import { EventEmitter } from 'events';
import { writeLog, writeOutput } from '../io';
import { inspect } from 'util';

export const Model = {
	'OpenAI_GPT4.1': 'openai/gpt-4.1',
	'OpenAI_GPT4.1-mini': 'openai/gpt-4.1-mini',
	'OpenAI_GPT4.1-nano': 'openai/gpt-4.1-nano',
	OpenAI_GPT4: 'openai/gpt-4',
	'OpenAI_GPT4-turbo': 'openai/gpt-4-turbo',
	OpenAI_GPT4o: 'openai/gpt-4o',
	'OpenAI_GPT4o-mini': 'openai/gpt-4o-mini',
	OpenAI_o3: 'openai/o3',
	'OpenAI_o4-mini': 'openai/o4-mini',
	OpenAI_o1: 'openai/o1',
	'OpenAI_o3-mini': 'openai/o3-mini',
	Anthropic_Claude_Haiku: 'anthropic/claude-3-haiku',
	Anthropic_Claude_Sonnet: 'anthropic/claude-3-sonnet',
	'Anthropic_Claude_Sonnet_3.7': 'anthropic/claude-3.7-sonnet',
	Anthropic_Claude_Opus: 'anthropic/claude-3-opus',
	Anthropic_Claude_4_Sonnet: 'anthropic/claude-sonnet-4',
	Anthropic_Claude_4_Opus: 'anthropic/claude-opus-4',
	Perplexity_Sonar: 'perplexity/sonar',
	Perplexity_Sonar_Pro: 'perplexity/sonar-pro',
	Perplexity_Sonar_Reasoning: 'perplexity/sonar-reasoning',
	Perplexity_Sonar_Reasoning_Pro: 'perplexity/sonar-reasoning-pro',
	Llama4_Scout: 'meta-llama/llama-4-scout',
	'Llama3.3_70B': 'meta-llama/llama-3.3-70b-instruct',
	'Llama3.1_8B': 'meta-llama/llama-3.1-8b-instruct',
	'Llama3.1_405B': 'meta-llama/llama-3.1-405b-instruct',
	Mistral_Nemo: 'mistralai/mistral-nemo',
	Mistral_Large: 'mistralai/mistral-large',
	Mistral_Medium: 'mistralai/mistral-medium-3',
	Mistral_Small: 'mistralai/mistral-small',
	Mistral_Codestral: 'mistralai/codestral-2501',
	'DeepSeek_R1_Distill_Llama_3.3_70B': 'deepseek/deepseek-r1-distill-llama-70b',
	DeepSeek_R1: 'deepseek/deepseek-r1',
	DeepSeek_V3: 'deepseek/deepseek-chat',
	'Google_Gemini_2.5_Pro': 'google/gemini-2.5-pro',
	'Google_Gemini_2.5_Flash': 'google/gemini-2.5-flash',
	'Google_Gemini_2.0_Flash': 'google/gemini-2.0-flash-001',
	xAI_Grok_3: 'x-ai/grok-3',
	xAI_Grok_3_Mini: 'x-ai/grok-3-mini',
	xAI_Grok_2: 'x-ai/grok-2-1212'
} as const;

export type Creativity = 'none' | 'low' | 'medium' | 'high' | 'maximum' | number;

export interface AskOptions {
	creativity?: Creativity;
	model?: keyof typeof Model;
	signal?: AbortSignal;
}

interface AskResult extends Promise<string> {
	on(event: 'data', listener: (chunk: string) => void): this;
	on(event: 'end', listener: (fullText: string) => void): this;
	on(event: 'error', listener: (error: Error) => void): this;
	off(event: 'data', listener: (chunk: string) => void): this;
	off(event: 'end', listener: (fullText: string) => void): this;
	off(event: 'error', listener: (error: Error) => void): this;
}

const pendingRequests = new Map<string, EventEmitter>();

export function handleAiStreamChunk(data: { requestId: string; text: string }) {
	const emitter = pendingRequests.get(data.requestId);
	if (emitter) {
		emitter.emit('data', data.text);
	}
}

export function handleAiStreamEnd(data: { requestId: string; full_text: string }) {
	const emitter = pendingRequests.get(data.requestId);
	if (emitter) {
		emitter.emit('end', data.full_text);
		pendingRequests.delete(data.requestId);
	}
}

export function handleAiStreamError(data: { requestId: string; error: string }) {
	const emitter = pendingRequests.get(data.requestId);
	if (emitter) {
		emitter.emit('error', new Error(data.error));
		pendingRequests.delete(data.requestId);
	}
}

export function ask(prompt: string, options: AskOptions = {}): AskResult {
	const emitter = new EventEmitter();
	const requestId = crypto.randomUUID();

	const modelMappings: Record<string, string> = {};
	if (options.model && Model[options.model]) {
		modelMappings[options.model] = Model[options.model];
	}

	let fullText = '';
	let isResolved = false;

	pendingRequests.set(requestId, emitter);

	const promise = new Promise<string>((resolve, reject) => {
		const handleChunk = (chunk: string) => {
			fullText += chunk;
		};

		const handleEnd = (finalText: string) => {
			if (!isResolved) {
				isResolved = true;
				fullText = finalText;
				resolve(fullText);
			}
		};

		const handleError = (error: Error) => {
			if (!isResolved) {
				isResolved = true;
				reject(error);
			}
		};

		emitter.on('data', handleChunk);
		emitter.on('end', handleEnd);
		emitter.on('error', handleError);

		if (options.signal) {
			options.signal.addEventListener('abort', () => {
				if (!isResolved) {
					isResolved = true;
					const error = new Error('Request aborted');
					emitter.emit('error', error);
					pendingRequests.delete(requestId);
				}
			});
		}

		writeOutput({
			type: 'ai-ask-stream',
			payload: {
				requestId,
				prompt,
				options: {
					model: options.model,
					creativity: options.creativity,
					modelMappings
				}
			}
		});
	});

	const result = promise as AskResult;
	result.on = emitter.on.bind(emitter);
	result.off = emitter.off.bind(emitter);

	return result;
}

export const AI = {
	ask,
	Model,
	Creativity: {
		none: 'none' as const,
		low: 'low' as const,
		medium: 'medium' as const,
		high: 'high' as const,
		maximum: 'maximum' as const
	}
};
