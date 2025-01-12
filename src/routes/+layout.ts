import { invoke } from '@tauri-apps/api/core';
import type { LayoutLoad } from './$types';

export const prerender = true;
export const ssr = false;

export const load: LayoutLoad = async () => ({
	isFFmpegAvailable: await invoke<boolean>('is_ffmpeg_available').catch((error) => {
		console.error(error);
		return false;
	})
});
