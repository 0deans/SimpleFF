<script lang="ts">
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { onDestroy } from 'svelte';

	interface CompressProgress {
		filePath: string;
		percentage: number;
	}

	export let filePath: string;
	let isCompressing = false;
	let compressProgress = 0;

	const unlisten = listen<string>('compress:progress', (event) => {
		const data = JSON.parse(event.payload) as CompressProgress;
		if (data.filePath !== filePath) return;
		compressProgress = data.percentage;
	});

	const compress = async () => {
		isCompressing = true;
		compressProgress = 0;
		await invoke('compress', {
			inputPath: filePath,
			outputPath: filePath.replace(/\.mp4$/, '_compressed.mp4')
		});
		isCompressing = false;
	};

	onDestroy(async () => {
		(await unlisten)();
	});
</script>

<div class="flex items-center justify-between space-x-2 rounded-md bg-gray-200 p-2">
	<h3 class="overflow-hidden">{filePath}</h3>
	<button
		on:click={compress}
		disabled={isCompressing}
		class="rounded-md border border-blue-400 px-2 py-1 transition enabled:hover:bg-blue-400 enabled:active:scale-95 disabled:border-gray-400"
	>
		{#if isCompressing}
			<div class="flex items-center space-x-1 min-w-16 justify-center">
				<span class="tabular-nums">{compressProgress.toFixed(0)}%</span>
				<Icon icon="mdi:loading" class="animate-spin" />
			</div>
		{:else}
			Compress
		{/if}
	</button>
</div>
