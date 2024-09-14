<script lang="ts">
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { cn, formatFileSize } from './utils';
	import { writable } from 'svelte/store';
	import { createProgress, melt } from '@melt-ui/svelte';
	import { listen } from '@tauri-apps/api/event';
	import { onDestroy } from 'svelte';

	interface CompressProgress {
		filePath: string;
		percentage: number;
	}

	export let path: string;
	export let onRemove: (path: string) => void;
	$: fileName = path.split(/[\\/]/).reverse()[0];
	$: outputPath = path.replace(/\.mp4$/, '_compressed.mp4');

	let isCompressing = false;
	let progress = writable(0);

	const {
		elements: { root },
		options: { max }
	} = createProgress({ value: progress, max: 100 });

	const get_file_size = async () => {
		return await invoke<number>('get_file_size', { path });
	};

	const unlisten = listen<string>('compress:progress', (event) => {
		const data = JSON.parse(event.payload) as CompressProgress;
		if (data.filePath !== path) return;
		progress.set(data.percentage);
	});

	const compress = async () => {
		isCompressing = true;
		progress.set(0);
		await invoke('compress', {
			inputPath: path,
			outputPath: outputPath
		});
		isCompressing = false;
	};

	const cancel = async () => {
		await invoke('cancel_compress', { outputPath });
		isCompressing = false;
	};

	onDestroy(async () => {
		(await unlisten)();
	});
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
					{/await}
				</span>
				<div class="flex items-center space-x-2">
					<button
						on:click={isCompressing ? cancel : compress}
						class={cn(
							'rounded-md bg-gray-100 px-2 font-medium text-blue-500 hover:bg-gray-200 hover:text-blue-700',
							isCompressing && 'bg-red-500 text-white hover:bg-red-600 hover:text-white'
						)}
					>
						{isCompressing ? 'cancel' : 'compress'}
					</button>
					<button
						on:click={() => onRemove(path)}
						disabled={isCompressing}
						class="rounded-md bg-gray-100 p-0.5 text-red-500 enabled:hover:bg-gray-200 enabled:hover:text-red-700 disabled:opacity-50"
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
			/>
		</div>
	{/if}
</div>
