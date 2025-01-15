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
	validation?: {
		pattern?: RegExp;
		min?: number;
		max?: number;
	};
}

export type CodecOption = CheckboxOption | SelectOption<string> | InputOption;
