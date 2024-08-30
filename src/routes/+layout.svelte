<script lang="ts">
	import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
	import { invalidateAll } from '$app/navigation';
	import type { PageData } from './$types';
	import Icon from '@iconify/svelte';
	import '../app.css';

	export let data: PageData;

	const appWindow = getCurrentWebviewWindow();
</script>

<div class="flex size-full flex-col overflow-hidden rounded-lg">
	<div
		data-tauri-drag-region
		class="flex w-full items-center justify-between bg-gray-600 text-white"
	>
		<h1 data-tauri-drag-region class="ml-2 touch-none select-none font-semibold">SimpleFF</h1>
		<div class="flex items-center">
			<button on:click={() => appWindow.minimize()} class="p-2 hover:bg-gray-500">
				<Icon icon="mdi:minimize" class="cursor-pointer" />
			</button>
			<button on:click={() => appWindow.close()} class="p-2 hover:bg-red-500">
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
	{/if}
</div>
