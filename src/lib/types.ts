import type { SelectOption as MSelectOption } from '@melt-ui/svelte';

export interface File {
	path: string;
	outputPath?: string;
	progress: number;
	isDone: boolean;
}

export interface CompressProgressPayload {
	filePath: string;
	percentage: number;
}

export interface OptionBase {
	name: string;
	ffmpegKey: string;
	type: 'checkbox' | 'select' | 'input';
	defaultValue?: string | number | boolean;
	description?: string;
}

export interface CheckboxOption extends OptionBase {
	type: 'checkbox';
	defaultValue?: boolean;
}

export interface SelectOption<T> extends OptionBase {
	type: 'select';
	options: T[];
}

export interface InputOption extends OptionBase {
	type: 'input';
	inputType: 'number' | 'text';
	defaultValue?: number | string;
	validation?: {
		pattern?: RegExp;
		min?: number;
		max?: number;
	};
}

export type CodecOption = CheckboxOption | SelectOption<string> | InputOption;
export type CodecParamValue<T = string> = string | number | boolean | MSelectOption<T> | undefined;
