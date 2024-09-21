import type { ClassValue } from 'clsx';
import clsx from 'clsx';
import { twMerge } from 'tailwind-merge';

export const cn = (...inputs: ClassValue[]) => {
	return twMerge(clsx(...inputs));
};

export const basename = (path: string, removeExtension = false) => {
	const parts = path.split(/[\\/]/);
	let base = parts[parts.length - 1];
	if (removeExtension) {
		const lastDotIndex = base.lastIndexOf('.');
		if (lastDotIndex !== -1) {
			base = base.substring(0, lastDotIndex);
		}
	}
	return base;
};

export const getFileExtension = (path: string) => {
	const lastDotIndex = path.lastIndexOf('.');
	if (lastDotIndex === -1) return '';
	return path.substring(lastDotIndex + 1);
};

export const dirname = (path: string) => {
	const parts = path.split(/[\\/]/);
	return parts.slice(0, parts.length - 1).join('\\');
};

export const formatFileSize = (bytes: number) => {
	const sizes = ['bytes', 'kb', 'mb', 'gb', 'tb'];
	if (bytes === 0) return '0 bytes';
	const i = Math.floor(Math.log(bytes) / Math.log(1024));
	return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${sizes[i]}`;
};
