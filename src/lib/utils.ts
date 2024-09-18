import type { ClassValue } from 'clsx';
import clsx from 'clsx';
import { twMerge } from 'tailwind-merge';

export const cn = (...inputs: ClassValue[]) => {
	return twMerge(clsx(...inputs));
};

export const basename = (path: string) => {
	const parts = path.split(/[\\/]/);
	return parts[parts.length - 1];
};

export const formatFileSize = (bytes: number) => {
	const sizes = ['bytes', 'kb', 'mb', 'gb', 'tb'];
	if (bytes === 0) return '0 bytes';
	const i = Math.floor(Math.log(bytes) / Math.log(1024));
	return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${sizes[i]}`;
};
