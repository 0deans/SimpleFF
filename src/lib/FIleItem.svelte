<script lang="ts">
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { basename, cn, formatFileSize } from './utils';
	import { writable } from 'svelte/store';
	import { createProgress, melt } from '@melt-ui/svelte';
	import { files, selectedFile } from './state.svelte';
	import { goto } from '$app/navigation';
	import type { File } from './types';

	export let file: File;
	$: fileName = basename(file.path);
	$: isCompressing = !!file.outputPath && !file.isDone;
	$: progress = writable(file.progress);

	const {
		elements: { root },
		options: { max }
	} = createProgress({ value: progress, max: 100 });

	const get_file_size = async () => {
		return await invoke<number>('get_file_size', { path: file.path });
	};

	const cancel = async () => {
		await invoke('cancel_compress', { outputPath: file.outputPath });
		files.update((files) => {
			const f = files.find((f) => f.path === file.path);
			if (f) f.outputPath = undefined;
			return files;
		});
	};

	const showInFolder = async () => {
		await invoke('show_in_folder', { path: file.outputPath });
	};

	const goNext = async () => {
		selectedFile.set(file);
		goto('/continue');
	};
</script>

<div class="rounded-lg border-2 border-gray-300 p-2">
	<div class="flex items-center space-x-2">
		<Icon icon="lets-icons:video-fill" class="size-8 text-gray-500" />
		<div class="w-full">
			<p class="break-all text-sm font-semibold">{fileName}</p>
			<div class="mt-0.5 flex items-center justify-between">
				<span class="text-sm text-gray-400">
					{#await get_file_size() then size}
						{formatFileSize(size)}
						<span class={cn(!isCompressing && 'hidden')}>
							| {$progress.toFixed(0)}%
						</span>
						{#if file.isDone}
							<span class="text-green-400">&#183; Done</span>
						{/if}
					{/await}
				</span>
				<div class="flex items-center space-x-2">
					{#if file.isDone}
						<button
							on:click={showInFolder}
							class="rounded-md bg-gray-100 p-0.5 text-orange-500 transition-transform hover:bg-gray-200 hover:text-orange-700 active:scale-95"
						>
							<Icon icon="bx:bx-folder-open" class="size-5" />
						</button>
					{/if}
					<button
						on:click={isCompressing ? cancel : goNext}
						class={cn(
							'rounded-md bg-gray-100 px-2 font-medium text-blue-500 transition-transform hover:bg-gray-200 hover:text-blue-700 active:scale-95',
							isCompressing && 'bg-red-500 text-white hover:bg-red-600 hover:text-white'
						)}
					>
						{isCompressing ? 'cancel' : 'continue'}
					</button>
					<button
						on:click={() => files.remove(file.path)}
						disabled={isCompressing}
						class="rounded-md bg-gray-100 p-0.5 text-red-500 transition-transform enabled:hover:bg-gray-200 enabled:hover:text-red-700 enabled:active:scale-95 disabled:opacity-50"
					>
						<Icon icon="bxs:trash" class="size-5" />
					</button>
				</div>
			</div>
		</div>
	</div>
	{#if isCompressing}
		<div use:melt={$root} class="mt-2 h-1 w-full overflow-hidden rounded-full bg-gray-200">
			<div
				class="size-full bg-blue-500 transition-transform duration-700 ease-[cubic-bezier(0.65,0,0.35,1)]"
				style={`transform: translateX(-${100 - (100 * ($progress ?? 0)) / ($max ?? 1)}%)`}
			></div>
		</div>
	{/if}
</div>
