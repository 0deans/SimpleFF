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

	export let data: PageData;
	let showWarning = false;

	interface CompressProgress {
		filePath: string;
		percentage: number;
	}

	const appWindow = getCurrentWebviewWindow();
	const unlisten = listen<string>('compress:progress', (event) => {
		const data = JSON.parse(event.payload) as CompressProgress;
		files.update((files) => {
			const file = files.find((f) => f.path === data.filePath);
			if (file) file.progress = data.percentage;
			return files;
		});
	});

	const unlisten2 = listen('close-requested', () => {
		showWarning = true;
	});

	const closeApp = () => invoke('close_request');

	onDestroy(async () => {
		(await unlisten)();
		(await unlisten2)();
	});
</script>

<div class="flex size-full flex-col overflow-hidden rounded-lg bg-white">
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
				>Retry</button
			>
		</div>
	{:else}
		<slot />

		{#if showWarning}
			<div
				class="fixed inset-0 flex items-center justify-center bg-gray-900/50 p-4 backdrop-blur-sm"
			>
				<div class="rounded-lg bg-white p-4 shadow-lg">
					<h2 class="font-semibold text-red-500">Warning</h2>
					<p class="text-gray-600">
						You have ongoing compressions. Closing the app will cancel all processes. Are you sure
						you want to close the app?
					</p>
					<div class="mt-4 flex justify-end space-x-2">
						<button on:click={closeApp} class="rounded-md bg-red-500 px-2 py-1 text-white">
							Yes, Cancel & Close
						</button>
						<button
							on:click={() => (showWarning = false)}
							class="rounded-md bg-gray-300 px-2 py-1 text-black"
						>
							No, Keep Running
						</button>
					</div>
				</div>
			</div>
		{/if}
	{/if}
</div>
