<script lang="ts">
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';

	export let filePath: string;
	let isCompressing = false;

	const compress = async () => {
		isCompressing = true;
		await invoke('compress', {
			inputPath: filePath,
			outputPath: filePath.replace(/\.mp4$/, '_compressed.mp4')
		});
		isCompressing = false;
	};
</script>

<div class="flex items-center justify-between space-x-2 rounded-md bg-gray-200 p-2">
	<h3 class="overflow-hidden">{filePath}</h3>
	<button
		on:click={compress}
		disabled={isCompressing}
		class=" rounded-md border border-blue-400 px-2 py-1 transition enabled:hover:bg-blue-400 enabled:active:scale-95 disabled:border-gray-400"
	>
		{#if isCompressing}
			<Icon icon="mdi:loading" class="h-6 w-16 animate-spin" />
		{:else}
			Compress
		{/if}
	</button>
</div>
