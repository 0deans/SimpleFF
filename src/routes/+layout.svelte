<script lang="ts">
	import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
	import { invalidateAll } from '$app/navigation';
	import type { PageData } from './$types';
	import Icon from '@iconify/svelte';
	import '../app.css';

	export let data: PageData;

	const appWindow = getCurrentWebviewWindow();
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
	{/if}
</div>
