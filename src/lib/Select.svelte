<script lang="ts">
	import Icon from '@iconify/svelte';
	import { melt, createSelect } from '@melt-ui/svelte';
	import { cn } from './utils';
	import type { SelectOption } from './types';

	interface Props {
		label?: string;
		options: SelectOption[];
		defaultSelected?: SelectOption;
		onSelect: (option: SelectOption) => void;
		className?: string;
	}

	let {
		label = 'Select an option',
		options,
		defaultSelected,
		onSelect,
		className = ''
	}: Props = $props();

	const {
		elements: { trigger, menu, option, label: labelElement },
		states: { open, selected },
		helpers: { isSelected }
	} = createSelect({
		forceVisible: true,
		defaultSelected,
		positioning: {
			placement: 'bottom',
			sameWidth: true
		}
	});

	$effect(() => {
		if (!$selected) return;
		onSelect($selected);
	});
</script>

<div class={cn('flex flex-col', className)}>
	<!-- svelte-ignore a11y_label_has_associated_control -->
	<label use:melt={$labelElement}>{label}</label>
	<button
		use:melt={$trigger}
		class="flex items-center justify-between rounded-md border-2 border-gray-200 px-3 py-2 outline-blue-200"
	>
		{$selected?.label || 'Select...'}
		<Icon icon="bi:chevron-down" class="size-4" />
	</button>
	{#if $open}
		<div
			use:melt={$menu}
			class="scroll-hidden z-10 flex max-h-[100px] flex-col overflow-y-auto rounded-lg border-2 border-gray-300 bg-white p-1 shadow focus:!ring-0"
		>
			{#each options as { value, label }}
				<div
					use:melt={$option({ value, label })}
					class="relative cursor-pointer rounded-lg py-1 pl-8 text-gray-600 hover:bg-blue-100 focus:z-10 focus:text-gray-800 data-[highlighted]:bg-blue-100"
				>
					{#if $isSelected(value)}
						<Icon
							icon="bi:check"
							class="absolute left-2 top-1/2 size-4 -translate-y-1/2 text-blue-500"
						/>
					{/if}
					{label}
				</div>
			{/each}
		</div>
	{/if}
</div>
