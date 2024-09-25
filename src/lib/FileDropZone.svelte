<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet<[
			{ isDragging: boolean }
		]>;
		onDrop: (paths: string[]) => void;
	}
	type PositionPayload = { position: { x: number; y: number } };
	type PathsPayload = { paths: string[] };

	let { onDrop, children }: Props = $props();
	let root: HTMLElement | null;
	let isDragging = $state(false);

	$effect(() => {
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

		return async () => {
			(await dragOver)();
			(await dragDrop)();
		};
	});
</script>

<div bind:this={root}>
	{@render children({isDragging})}
</div>
