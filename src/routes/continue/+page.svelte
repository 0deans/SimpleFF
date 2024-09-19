<script lang="ts">
	import { goto } from '$app/navigation';
	import { files } from '$lib/state.svelte';
	import { basename } from '$lib/utils';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { PageData } from './$types';

	export let data: PageData;
	const { selectedFile } = data;
	const output = selectedFile.path.replace(/\.mp4$/, '_compressed.mp4');

	const compress = async () => {
		files.update((files) => {
			const file = files.find((f) => f.path === selectedFile.path);
			if (file) {
				file.outputPath = output;
				file.progress = 0;
				file.isDone = false;
			}
			return files;
		});

		invoke<boolean>('compress', {
			inputPath: selectedFile.path,
			outputPath: output
		}).then((success) => {
			if (!success) return;
			files.update((files) => {
				const file = files.find((f) => f.path === selectedFile.path);
				if (file) file.isDone = true;
				return files;
			});
		});

		goto('/', { replaceState: true });
	};
</script>

<main class="flex size-full flex-col p-3">
	<div class="flex items-center space-x-2">
		<a href="/" class="rounded-md p-1 hover:bg-gray-200">
			<Icon icon="ep:back" class="size-6" />
		</a>
		<span>Return</span>
	</div>

	<div class="flex-auto">
		<h1 class="my-2 text-blue-400">{basename(selectedFile.path)}</h1>
	</div>

	<button
		on:click={compress}
		class="w-full rounded-lg bg-blue-600 p-2 text-white transition-transform hover:bg-blue-700 active:scale-95"
	>
		Compress
	</button>
</main>
