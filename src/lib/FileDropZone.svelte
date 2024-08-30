<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { onDestroy } from 'svelte';

	export let onDrop: (paths: string[]) => void;

	let root: HTMLElement | null;
	let isDragging = false;

	type PositionPayload = { position: { x: number; y: number } };
	type PathsPayload = { paths: string[] };

	const dragOver = listen<PositionPayload>('tauri://drag-over', (e) => {
		const position = e.payload.position;
		const hoveredElement = document.elementFromPoint(position.x, position.y);
		isDragging = !!root && root.contains(hoveredElement);
	});

	const dragDrop = listen<PositionPayload & PathsPayload>('tauri://drag-drop', (e) => {
		const position = e.payload.position;
		const hoveredElement = document.elementFromPoint(position.x, position.y);
		if (root && root.contains(hoveredElement)) {
			isDragging = false;
			onDrop(e.payload.paths);
		}
	});

	onDestroy(async () => {
		(await dragOver)();
		(await dragDrop)();
	});
</script>

<div bind:this={root}>
	<slot {isDragging} />
</div>
