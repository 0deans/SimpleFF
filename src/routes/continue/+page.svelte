<script lang="ts">
	import { goto } from '$app/navigation';
	import { basename, cn, dirname, getFileExtension } from '$lib/utils';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { PageData } from './$types';
	import { open } from '@tauri-apps/plugin-dialog';
	import { melt, createLabel, createScrollArea, type SelectOption } from '@melt-ui/svelte';
	import Select from '$lib/Select.svelte';
	import { fileStore } from '$lib/state.svelte';
	import {
		audioCodecOptions,
		extensionOptions,
		formatCodecs,
		noneOption,
		videoCodecConfig,
		videoCodecOptions
	} from '$lib/constants';
	import Checkbox from '$lib/Checkbox.svelte';

	let { data }: { data: PageData } = $props();
	const { selectedFile } = data;

	let matchingExtension = extensionOptions.find((ext) => {
		return ext.value === getFileExtension(selectedFile.path);
	});

	let outputDirname = $state(dirname(selectedFile.path));
	let filename = $state(`${basename(selectedFile.path, true)}_compressed`);
	let extension = $state(matchingExtension || extensionOptions[0]);
	let isFilenameValid = $state(true);

	let codecOptions = $derived.by(() => {
		const { video, audio } = formatCodecs[extension.value];

		return {
			video: [noneOption, ...videoCodecOptions.filter(({ value }) => video.includes(value))],
			audio: [noneOption, ...audioCodecOptions.filter(({ value }) => audio.includes(value))]
		};
	});

	let videoCodec = $state<SelectOption>(noneOption);
	let audioCodec = $state<SelectOption>(noneOption);

	$effect(() => {
		const isAvailable = codecOptions.video.some(({ value }) => value === videoCodec?.value);
		if (!isAvailable) videoCodec = codecOptions.video[0];
	});

	$effect(() => {
		const isAvailable = codecOptions.audio.some(({ value }) => value === audioCodec?.value);
		if (!isAvailable) audioCodec = codecOptions.audio[0];
	});

	const {
		elements: { root }
	} = createLabel();

	const {
		elements: {
			root: scrollRoot,
			content: scrollContent,
			viewport: scrollViewport,
			scrollbarY,
			thumbY
		}
	} = createScrollArea();

	const selectOutputFolder = async () => {
		const selected = await open({
			directory: true,
			defaultPath: outputDirname
		});

		if (selected) outputDirname = selected;
	};

	const compress = async () => {
		const outputPath = `${outputDirname}\\${filename}.${extension.value}`;

		fileStore.update(selectedFile.path, (file) => {
			file.outputPath = outputPath;
			file.progress = 0;
			file.isDone = false;
			return file;
		});

		const params = {
			inputPath: selectedFile.path,
			outputPath: outputPath,
			videoCodec: videoCodec?.value,
			audioCodec: audioCodec?.value
		};

		invoke<boolean>('compress', { params })
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

<main class="flex size-full flex-col">
	<div class="flex items-center space-x-2 p-3 pb-0">
		<a href="/" class="rounded-md p-1 hover:bg-gray-200">
			<Icon icon="ep:back" class="size-6" />
		</a>
		<span>Return</span>
	</div>

	<div class="h-0 flex-shrink-0 flex-grow basis-auto">
		<div use:melt={$scrollRoot} class="size-full overflow-hidden">
			<div use:melt={$scrollViewport} class="size-full">
				<div use:melt={$scrollContent} class="!block space-y-4 p-3">
					<h1 class="text-blue-400">{basename(selectedFile.path)}</h1>

					<div class="flex flex-col">
						<label use:melt={$root} for="output-path">Output path</label>
						<div class="flex justify-between rounded-md border-2 border-gray-200">
							<p class="scroll-hidden w-full overflow-x-auto whitespace-nowrap p-2">
								{outputDirname}
							</p>
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
								onchange={() => (isFilenameValid = filename.length > 0 && filename.length <= 200)}
								type="text"
								id="filename"
								autocomplete="off"
								class={cn(
									'w-full rounded-md border-2 border-gray-200 p-2 outline-blue-200',
									!isFilenameValid && 'border-red-500'
								)}
							/>
							<p class={cn('text-red-400', isFilenameValid && 'hidden')}>
								Must be 1-200 characters long
							</p>
						</div>
						<Select
							options={extensionOptions}
							bind:selected={extension}
							label="Extension"
							className="w-1/2"
						/>
					</div>

					<div class="flex space-x-2">
						<Select
							options={codecOptions.video}
							bind:selected={videoCodec}
							label="Video Codec"
							className="w-1/2"
						/>
						<Select
							options={codecOptions.audio}
							bind:selected={audioCodec}
							label="Audio Codec"
							className="w-1/2"
						/>
					</div>

					{#if videoCodec.value}
						<div class="space-y-4">
							<div>
								<h2 class="text-blue-600">Video Codec Options</h2>
								<div class="border-t border-gray-400"></div>
							</div>

							{#if typeof videoCodec.value === 'string'}
								{#each videoCodecConfig[videoCodec.value] as item}
									{#if item.type === 'checkbox'}
										<Checkbox label={item.name} checked />
									{:else if item.type === 'select'}
										<Select
											options={item.options.map((value) => {
												return { value, label: value };
											})}
											label={item.name}
										/>
									{/if}
								{/each}
							{/if}
						</div>
					{/if}

					{#if audioCodec.value}
						<div>
							<h2 class="text-blue-600">Audio Codec Options</h2>
							<div class="border-t border-gray-400"></div>
						</div>
					{/if}
				</div>
			</div>
			<div
				use:melt={$scrollbarY}
				class="flex h-[95%] w-2 touch-none select-none rounded-full border-l border-l-transparent bg-gray-900/10 p-px transition-colors"
			>
				<div use:melt={$thumbY} class="relative flex-1 rounded-full bg-gray-600"></div>
			</div>
		</div>
	</div>

	<button
		onclick={compress}
		disabled={!isFilenameValid}
		class="w-full bg-blue-600 p-2 text-white transition-transform enabled:hover:bg-blue-700 disabled:opacity-60"
	>
		Process
	</button>
</main>
