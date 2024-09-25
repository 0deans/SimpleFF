<script lang="ts">
	import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
	import { listen } from '@tauri-apps/api/event';
	import { invalidateAll } from '$app/navigation';
	import type { PageData } from './$types';
	import Icon from '@iconify/svelte';
	import '../app.css';
	import { onDestroy } from 'svelte';
	import { files } from '$lib/state.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { CompressProgressPayload } from '$lib/types';
	import { melt, createDialog } from '@melt-ui/svelte';

	export let data: PageData;

	const {
		elements: { overlay, content, title, description, close },
		states: { open }
	} = createDialog({ role: 'alertdialog', forceVisible: true });

	const appWindow = getCurrentWebviewWindow();
	const unlisten = listen<string>('compress:progress', (event) => {
		const data = JSON.parse(event.payload) as CompressProgressPayload;
		files.updateFile(data.filePath, (file) => {
			file.progress = data.percentage;
			return file;
		});
	});

	const unlisten2 = listen('close-requested', () => {
		open.set(true);
	});

	const closeApp = () => invoke('close_request');

	onDestroy(async () => {
		(await unlisten)();
		(await unlisten2)();
	});
</script>

<div class="relative flex size-full flex-col overflow-hidden rounded-lg bg-white">
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div
		on:mousedown={(e) => {
			if (e.buttons !== 1) return;
			appWindow.startDragging();
		}}
		class="flex w-full items-center justify-between border-b border-gray-300"
	>
		<div class="ml-2 flex items-center space-x-1">
			<Icon icon="logos:ffmpeg-icon" />
			<h1 class="touch-none select-none font-semibold">SimpleFF</h1>
		</div>
		<div class="flex items-center">
			<button
				on:mousedown|stopPropagation
				on:click={() => appWindow.minimize()}
				class="p-2 hover:bg-gray-200"
			>
				<Icon icon="mdi:minimize" class="cursor-pointer" />
			</button>
			<button
				on:mousedown|stopPropagation
				on:click={() => appWindow.close()}
				class="p-2 hover:bg-red-500"
			>
				<Icon icon="mdi:close" class="cursor-pointer" />
			</button>
		</div>
	</div>

	{#if !data.isFFmpegAvailable}
		<div
			class="flex h-full flex-col items-center justify-center space-y-4 text-balance rounded-md p-4 text-center font-medium text-red-500"
		>
			<Icon icon="material-symbols:error" width={48} height={48} />
			<p>FFmpeg is not available. Please ensure it is installed and accessible.</p>
			<button
				on:click={() => invalidateAll()}
				class="rounded-md border border-blue-500 px-4 py-2 text-blue-500 hover:bg-blue-500 hover:text-white active:scale-95"
			>
				Retry
			</button>
		</div>
	{:else}
		<slot />
	{/if}

	{#if $open}
		<div>
			<div use:melt={$overlay} class="absolute inset-0 z-50 bg-black/50"></div>
			<div
				use:melt={$content}
				class="absolute left-1/2 top-1/2 z-50 max-h-[85vh] w-[90vw] -translate-x-1/2 -translate-y-1/2 rounded-lg bg-white p-4 shadow-lg"
			>
				<h2 use:melt={$title} class="text-lg font-medium text-red-600">Warning</h2>
				<p use:melt={$description} class="mb-5 mt-2 leading-normal text-zinc-600">
					You have ongoing compressions. Closing the app will cancel all processes. Are you sure you
					want to close the app?
				</p>

				<div class="mt-6 flex justify-end space-x-4">
					<button
						use:melt={$close}
						class="inline-flex rounded-md bg-zinc-100 px-4 font-medium text-zinc-600"
					>
						No, Keep Open
					</button>
					<button
						on:click={closeApp}
						class="inline-flex rounded-md bg-red-100 px-4 font-medium text-red-600"
					>
						Yes, Cancel & Close
					</button>
				</div>

				<button
					use:melt={$close}
					aria-label="close"
					class="absolute right-4 top-4 appearance-none rounded-md hover:bg-gray-100"
				>
					<Icon icon="mdi:close" class="size-6" />
				</button>
			</div>
		</div>
	{/if}
</div>
