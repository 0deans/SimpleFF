import { get } from 'svelte/store';
import type { PageLoad } from './$types';
import { redirect } from '@sveltejs/kit';
import { selectedFile } from '$lib/state.svelte';

export const load: PageLoad = () => {
	const file = get(selectedFile);
	if (!file) redirect(303, '/');

	return { selectedFile: file };
};
