<script lang="ts">
	import { goto } from '$app/navigation';
	import { files } from '$lib/state.svelte';
	import { basename, cn, dirname, getFileExtension } from '$lib/utils';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { PageData } from './$types';
	import { open } from '@tauri-apps/plugin-dialog';
	import { melt, createLabel } from '@melt-ui/svelte';
	import type { SelectOption } from '$lib/types';
	import Select from '$lib/Select.svelte';

	export let data: PageData;
	const { selectedFile } = data;

	let outputDirname = dirname(selectedFile.path);
	let filename = `${basename(selectedFile.path, true)}_compressed`;
	let extension = getFileExtension(selectedFile.path);
	let isValid = true;

	const audioCodecs: SelectOption[] = [
		{ value: 'aac', label: 'AAC' },
		{ value: 'mp3', label: 'MP3' },
		{ value: 'ac3', label: 'AC-3' },
		{ value: 'alac', label: 'ALAC' },
		{ value: 'opus', label: 'Opus' },
		{ value: 'pcm', label: 'PCM' },
		{ value: 'vorbis', label: 'Vorbis' },
		{ value: 'dts', label: 'DTS' },
		{ value: 'flac', label: 'FLAC' },
		{ value: 'mp2', label: 'MP2' },
		{ value: 'mp1', label: 'MP1' }
	];

	const videoCodecs: SelectOption[] = [
		{ value: 'h264', label: 'H.264 (AVC)' },
		{ value: 'h265', label: 'H.265 (HEVC)' },
		{ value: 'vp9', label: 'VP9' },
		{ value: 'av1', label: 'AV1' },
		{ value: 'mpeg4', label: 'MPEG-4 Part 2' },
		{ value: 'prores', label: 'ProRes' },
		{ value: 'xvid', label: 'Xvid' },
		{ value: 'divx', label: 'DivX' },
		{ value: 'mjpeg', label: 'MJPEG' },
		{ value: 'h263', label: 'H.263' },
		{ value: 'mpeg2', label: 'MPEG-2' },
		{ value: 'mpeg1', label: 'MPEG-1' },
		{ value: 'theora', label: 'Theora' }
	];

	const extensions: SelectOption[] = [
		{ value: 'mp4', label: 'MP4' },
		{ value: 'mkv', label: 'MKV' },
		{ value: 'avi', label: 'AVI' },
		{ value: 'mov', label: 'MOV' },
		{ value: 'webm', label: 'WEBM' },
		{ value: 'mpg', label: 'MPG' },
		{ value: 'mpeg', label: 'MPEG' },
		{ value: 'ogv', label: 'OGV' }
	];

	let selectedExtension = extensions.find((ext) => ext.value === extension) || extensions[0];
	let selectedVideoCodec = videoCodecs[0];
	let selectedAudioCodec = audioCodecs[0];

	const {
		elements: { root }
	} = createLabel();

	const selectOutputFolder = async () => {
		const selected = await open({
			directory: true,
			defaultPath: outputDirname
		});

		if (selected) outputDirname = selected;
	};

	const compress = async () => {
		const outputPath = `${outputDirname}\\${filename}.${selectedExtension.value}`;

		files.updateFile(selectedFile.path, (file) => {
			file.outputPath = outputPath;
			file.progress = 0;
			file.isDone = false;
			return file;
		});

		invoke<boolean>('compress', {
			inputPath: selectedFile.path,
			outputPath: outputPath
		})
			.then((success) => {
				if (!success) return;
				files.updateFile(selectedFile.path, (file) => {
					file.isDone = true;
					return file;
				});
			})
			.catch((e) => {
				console.error(e);
				files.updateFile(selectedFile.path, (file) => {
					file.outputPath = undefined;
					return file;
				});
			});

		goto('/', { replaceState: true });
	};
</script>

<main class="flex size-full flex-col p-3">
	<div class="flex items-center space-x-2">
		<a href="/" class="rounded-md p-1 hover:bg-gray-200">
			<Icon icon="ep:back" class="size-6" />
		</a>
		<span>Return</span>
	</div>

	<div class="my-2 space-y-4 overflow-y-auto">
		<h1 class="text-blue-400">{basename(selectedFile.path)}</h1>

		<div class="flex flex-col">
			<label use:melt={$root} for="output-path">Output path</label>
			<div class="flex justify-between rounded-md border-2 border-gray-200">
				<p class="scroll-hidden w-full overflow-x-auto whitespace-nowrap p-2">{outputDirname}</p>
				<button
					on:click={selectOutputFolder}
					class="group p-2 text-blue-500 outline-blue-200 hover:bg-gray-200 hover:text-blue-700"
				>
					<Icon
						icon="material-symbols:folder"
						class="size-6 transition-transform group-active:scale-95"
					/>
				</button>
			</div>
		</div>

		<div class="flex space-x-2">
			<div class="flex w-full flex-col">
				<label use:melt={$root} for="filename">Output file name</label>
				<input
					bind:value={filename}
					on:change={() => (isValid = filename.length > 0 && filename.length <= 200)}
					type="text"
					id="filename"
					autocomplete="off"
					class={cn(
						'w-full rounded-md border-2 border-gray-200 p-2 outline-blue-200',
						!isValid && 'border-red-500'
					)}
				/>
				<p class={cn('text-red-400', isValid && 'hidden')}>Must be 1-200 characters long</p>
			</div>
			<Select
				options={extensions}
				defaultSelected={selectedExtension}
				label="Extension"
				on:select={(e) => (selectedExtension = e.detail)}
				className="w-1/2"
			/>
		</div>

		<div class="flex space-x-2">
			<Select
				options={videoCodecs}
				defaultSelected={selectedVideoCodec}
				label="Video Codec"
				on:select={(e) => (selectedVideoCodec = e.detail)}
				className="w-1/2"
			/>
			<Select
				options={audioCodecs}
				defaultSelected={selectedAudioCodec}
				label="Audio Codec"
				on:select={(e) => (selectedAudioCodec = e.detail)}
				className="w-1/2"
			/>
		</div>

		<button
			on:click={compress}
			disabled={!isValid}
			class="w-full rounded-lg bg-blue-600 p-2 text-white transition-transform enabled:hover:bg-blue-700 enabled:active:scale-95 disabled:opacity-60"
		>
			Process
		</button>
	</div>
</main>
