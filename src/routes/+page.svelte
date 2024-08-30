<script lang="ts">
	import { createScrollArea, melt } from '@melt-ui/svelte';
	import { cn } from '$lib/utils';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import FileDropZone from '$lib/FileDropZone.svelte';

	const {
		elements: { root, content, viewport, scrollbarY, thumbY }
	} = createScrollArea();
	let filePaths: string[] | null;

	const handleSelect = async () => {
		filePaths = await open({
			multiple: true,
			directory: false,
			filters: [
				{
					name: 'Video',
					extensions: ['mp4', 'mkv', 'avi', 'mov', 'webm', 'mpg', 'mpeg', 'ogv']
				}
			]
		});
	};

	const compress = async (filePath: string) => {
		const result = await invoke('compress', {
			inputPath: filePath,
			outputPath: filePath.replace(/\.mp4$/, '_compressed.mp4')
		});
		console.log(result);
	};
</script>

<main use:melt={$root} class="size-full overflow-hidden bg-gray-100">
	<div use:melt={$viewport} class="size-full">
		<div use:melt={$content} class="!block p-2 px-3">
			<FileDropZone let:isDragging onDrop={(paths) => (filePaths = paths)}>
				<button
					on:click={handleSelect}
					class={cn(
						'grid h-44 w-full content-center rounded-md border-2 border-dashed border-gray-500 transition hover:bg-gray-200 active:scale-95',
						{ 'bg-gray-200': isDragging }
					)}
				>
					<Icon icon="material-symbols:upload" class="mx-auto text-4xl text-gray-500" />
					Drop Video here
					<span class="text-gray-400">- or -</span>
					Click to select
				</button>
			</FileDropZone>

			{#if filePaths}
				<div class="my-2 space-y-2">
					{#each filePaths as path}
						<div class="flex items-center justify-between space-x-2 rounded-md bg-gray-200 p-2">
							<h3 class="overflow-hidden">{path}</h3>
							<button
								on:click={() => compress(path)}
								class="rounded-md border border-blue-400 px-2 py-1 transition hover:bg-blue-400 active:scale-95"
								>Compress</button
							>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	</div>
	<div
		use:melt={$scrollbarY}
		class="flex h-full w-2 touch-none select-none border-l border-l-transparent bg-orange-800/10 p-px transition-colors"
	>
		<div use:melt={$thumbY} class="relative flex-1 rounded-full bg-orange-600" />
	</div>
</main>
