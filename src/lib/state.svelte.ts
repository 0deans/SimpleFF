import { writable } from 'svelte/store';
import type { File } from './types';

const createFileStore = () => {
	const files = $state<File[]>([]);

	const add = (file: File) => {
		if (files.some((f) => f.path === file.path)) return;
		files.push(file);
	};

	const update = (path: string, updateFn: (file: File) => File) => {
		const index = files.findIndex((f) => f.path === path);
		if (index === -1) return;
		files[index] = updateFn(files[index]);
	};

	const remove = (path: string) => {
		const index = files.findIndex((f) => f.path === path);
		if (index === -1) return;
		files.splice(index, 1);
	};

	return {
		get files() {
			return files;
		},
		add,
		update,
		remove
	};
};

export const fileStore = createFileStore();
export const selectedFile = writable<File | null>(null);
