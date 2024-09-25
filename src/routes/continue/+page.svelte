<script lang="ts">
	import { goto } from '$app/navigation';
	import { basename, cn, dirname, getFileExtension } from '$lib/utils';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { PageData } from './$types';
	import { open } from '@tauri-apps/plugin-dialog';
	import { melt, createLabel } from '@melt-ui/svelte';
	import Select from '$lib/Select.svelte';
	import { fileStore } from '$lib/state.svelte';
	import {
		audioCodecOptions,
		extensionOptions,
		formatCodecs,
		videoCodecOptions
	} from '$lib/constants';

	let { data }: { data: PageData } = $props();
	const { selectedFile } = data;

	let outputDirname = $state(dirname(selectedFile.path));
	let filename = $state(`${basename(selectedFile.path, true)}_compressed`);
	let extension = getFileExtension(selectedFile.path);
	let isValid = $state(true);

	let matchingExtension = extensionOptions.find((ext) => ext.value === extension);
	let selectedExtension = $state(matchingExtension || extensionOptions[0]);

	let codecs = $derived.by(() => {
		const codecs = formatCodecs[selectedExtension.value];

		const video = videoCodecOptions.filter((codec) => codecs.video.includes(codec.value));
		const audio = audioCodecOptions.filter((codec) => codecs.audio.includes(codec.value));

		return { video, audio };
	});

	let selectedVideoCodec = $state(videoCodecOptions[0]);
	let selectedAudioCodec = $state(audioCodecOptions[0]);

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

		fileStore.update(selectedFile.path, (file) => {
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
				fileStore.update(selectedFile.path, (file) => {
					file.isDone = true;
					return file;
				});
			})
			.catch((e) => {
				console.error(e);
				fileStore.update(selectedFile.path, (file) => {
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
					onclick={selectOutputFolder}
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
					onchange={() => (isValid = filename.length > 0 && filename.length <= 200)}
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
				options={extensionOptions}
				defaultSelected={selectedExtension}
				label="Extension"
				onSelect={(option) => (selectedExtension = option)}
				className="w-1/2"
			/>
		</div>

		<div class="flex space-x-2">
			<Select
				options={codecs.video}
				defaultSelected={selectedVideoCodec}
				label="Video Codec"
				onSelect={(option) => (selectedVideoCodec = option)}
				className="w-1/2"
			/>
			<Select
				options={codecs.audio}
				defaultSelected={selectedAudioCodec}
				label="Audio Codec"
				onSelect={(option) => (selectedAudioCodec = option)}
				className="w-1/2"
			/>
		</div>

		<button
			onclick={compress}
			disabled={!isValid}
			class="w-full rounded-lg bg-blue-600 p-2 text-white transition-transform enabled:hover:bg-blue-700 enabled:active:scale-95 disabled:opacity-60"
		>
			Process
		</button>
	</div>
</main>
