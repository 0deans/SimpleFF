import { invoke } from '@tauri-apps/api/core';
import type { LayoutLoad } from './$types';

export const prerender = true;
export const ssr = false;

export const load: LayoutLoad = async () => {
	const isFFmpegAvailable = await invoke('is_ffmpeg_available');
	return { isFFmpegAvailable };
};
