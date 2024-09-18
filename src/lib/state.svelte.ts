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

	const remove = (path: string) =>
		update((oldFiles) => oldFiles.filter((file) => file.path !== path));

	return { subscribe, update, add, remove };
};

export const files = createFileStore();
