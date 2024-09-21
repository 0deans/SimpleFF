import { writable } from 'svelte/store';
import type { File } from './types';

const createFileStore = () => {
	const { subscribe, update } = writable<File[]>([]);

	const add = (newFiles: File[]) =>
		update((oldFiles) => {
			const filesMap = new Map(oldFiles.map((file) => [file.path, file]));
			newFiles.forEach((file) => filesMap.set(file.path, file));
			return Array.from(filesMap.values());
		});

	const updateFile = (path: string, updateFn: (file: File) => File) =>
		update((oldFiles) => {
			return oldFiles.map((file) => {
				if (file.path === path) {
					return updateFn(file);
				}
				return file;
			});
		});

	const remove = (path: string) =>
		update((oldFiles) => oldFiles.filter((file) => file.path !== path));

	return { subscribe, update, add, updateFile, remove };
};

export const files = createFileStore();
export const selectedFile = writable<File | null>(null);
