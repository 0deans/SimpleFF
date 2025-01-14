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
