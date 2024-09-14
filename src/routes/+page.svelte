<script lang="ts">
	import { createScrollArea, melt } from '@melt-ui/svelte';
	import { cn, joinFilePaths } from '$lib/utils';
	import Icon from '@iconify/svelte';
	import { open } from '@tauri-apps/plugin-dialog';
	import FileDropZone from '$lib/FileDropZone.svelte';
	import FileItem from '$lib/FileItem.svelte';

	const {
		elements: { root, content, viewport, scrollbarY, thumbY }
	} = createScrollArea();
	let filePaths: string[] | null;

	const handleSelect = async () => {
		const newPaths = await open({
			multiple: true,
			directory: false,
			filters: [
				{
					name: 'Video',
					extensions: ['mp4', 'mkv', 'avi', 'mov', 'webm', 'mpg', 'mpeg', 'ogv']
				}
			]
		});
		if (newPaths === null) return;

		filePaths = joinFilePaths({
			newPaths,
			oldPaths: filePaths
		});
	};

	const onRemove = (filePath: string) => {
		filePaths = filePaths?.filter((path) => path !== filePath) ?? null;
	};
</script>

<main use:melt={$root} class="size-full overflow-hidden">
	<div use:melt={$viewport} class="size-full">
		<div use:melt={$content} class="!block p-3">
			<FileDropZone
				let:isDragging
				onDrop={(paths) => {
					filePaths = joinFilePaths({
						newPaths: paths,
						oldPaths: filePaths
					});
				}}
			>
				<div
					class={cn(
						'flex h-36 flex-col items-center justify-center space-y-2 rounded-lg border border-green-400 bg-green-400/10',
						{ 'bg-green-400/20': isDragging }
					)}
				>
					<Icon icon="mingcute:file-upload-fill" class="size-8 text-green-700" />
					<span class="font-medium">Drag & drop your files here or</span>
					<button
						on:click={handleSelect}
						class="rounded-md bg-gray-400/50 px-4 py-2 font-medium transition hover:bg-gray-400/70 active:scale-95"
					>
						Choose files
					</button>
				</div>
			</FileDropZone>
			<p class="mt-2 text-sm font-medium text-gray-600">Only video files are supported.</p>

			{#if !filePaths || filePaths.length === 0}
				<div class="mt-20 flex items-center justify-center text-xl">
					<Icon icon="twemoji:melting-face" />
					There are no files
					<Icon icon="twemoji:melting-face" />
				</div>
			{:else}
				<h2 class="mt-4 text-lg font-medium">Selected Files</h2>
				<div class="mt-2 space-y-2">
					{#each filePaths as path (path)}
						<FileItem {path} {onRemove} />
					{/each}
				</div>
			{/if}
		</div>
	</div>
	<div
		use:melt={$scrollbarY}
		class="flex h-full w-2 touch-none select-none border-l border-l-transparent bg-gray-900/10 p-px transition-colors"
	>
		<div use:melt={$thumbY} class="relative flex-1 rounded-full bg-gray-600" />
	</div>
</main>
