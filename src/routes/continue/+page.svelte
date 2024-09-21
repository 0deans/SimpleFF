<script lang="ts">
	import { goto } from '$app/navigation';
	import { files } from '$lib/state.svelte';
	import { basename, cn, dirname, getFileExtension } from '$lib/utils';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { PageData } from './$types';
	import { open } from '@tauri-apps/plugin-dialog';
	import { melt, createSelect, createLabel } from '@melt-ui/svelte';

	export let data: PageData;
	const { selectedFile } = data;

	let outputDirname = dirname(selectedFile.path);
	let filename = `${basename(selectedFile.path, true)}_compressed`;
	let extension = getFileExtension(selectedFile.path);
	let isValid = true;

	const extensions = [
		{ value: 'mp4', label: 'MP4' },
		{ value: 'mkv', label: 'MKV' },
		{ value: 'avi', label: 'AVI' },
		{ value: 'mov', label: 'MOV' },
		{ value: 'webm', label: 'WEBM' },
		{ value: 'mpg', label: 'MPG' },
		{ value: 'mpeg', label: 'MPEG' },
		{ value: 'ogv', label: 'OGV' }
	];
	const defaultExtension = extensions.find((ext) => ext.value === extension);

	const {
		elements: { root }
	} = createLabel();

	const {
		elements: { trigger, menu, option, label: selectLabel },
		states: { open: extensionSelectOpen, selected: selectedExtension },
		helpers: { isSelected }
	} = createSelect<string>({
		forceVisible: true,
		defaultSelected: defaultExtension ?? extensions[0],
		positioning: {
			placement: 'bottom',
			sameWidth: true
		}
	});

	const selectOutputFolder = async () => {
		const selected = await open({
			directory: true,
			defaultPath: outputDirname
		});

		if (selected) outputDirname = selected;
	};

	const compress = async () => {
		const outputPath = `${outputDirname}\\${filename}.${$selectedExtension!.value}`;

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

	<div class="flex-auto">
		<h1 class="my-2 text-blue-400">{basename(selectedFile.path)}</h1>

		<div class="space-y-4">
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
					<p class={cn('text-red-400', isValid && 'invisible')}>Must be 1-200 characters long</p>
				</div>

				<div class="flex flex-col">
					<!-- svelte-ignore a11y-label-has-associated-control -->
					<label use:melt={$selectLabel} class="group block">Extension</label>
					<button
						use:melt={$trigger}
						class="flex w-24 items-center justify-between rounded-md border-2 border-gray-200 p-2 px-3 outline-blue-200"
					>
						{$selectedExtension?.label}
						<Icon icon="mdi:chevron-down" class="size-6" />
					</button>
					{#if $extensionSelectOpen}
						<div
							use:melt={$menu}
							class="scroll-hidden z-10 flex max-h-[100px] flex-col overflow-y-auto rounded-lg border-2 border-gray-300 bg-white p-1 shadow focus:!ring-0"
						>
							{#each extensions as { value, label }}
								<div
									use:melt={$option({ value: value, label: label })}
									class="relative cursor-pointer rounded-lg py-1 pl-8 text-gray-600 hover:bg-blue-100 focus:z-10 focus:text-gray-800"
								>
									{#if $isSelected(value)}
										<Icon
											icon="mdi:check"
											class="absolute left-2 top-1/2 size-4 -translate-y-1/2 text-blue-500"
										/>
									{/if}
									{label}
								</div>
							{/each}
						</div>
					{/if}
				</div>
			</div>
		</div>
	</div>

	<button
		on:click={compress}
		disabled={!isValid}
		class="w-full rounded-lg bg-blue-600 p-2 text-white transition-transform enabled:hover:bg-blue-700 enabled:active:scale-95 disabled:opacity-60"
	>
		Compress
	</button>
</main>
