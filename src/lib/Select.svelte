<script lang="ts">
	import Icon from '@iconify/svelte';
	import { melt, createSelect, type SelectOption } from '@melt-ui/svelte';
	import { cn } from './utils';

	interface Props {
		label?: string;
		options: SelectOption[];
		selected?: SelectOption;
		className?: string;
	}

	let { label, options, selected = $bindable(), className = '' }: Props = $props();

	const {
		elements: { trigger, menu, option, label: labelElement },
		states: { open, selected: localSelected },
		helpers: { isSelected }
	} = createSelect({
		forceVisible: true,
		defaultSelected: selected,
		positioning: {
			placement: 'bottom',
			sameWidth: true
		}
	});

	$effect(() => localSelected.set(selected));
	$effect(() => {
		if (!$localSelected) return;
		selected = $localSelected;
	});
</script>

<div class={cn('flex flex-col', className)}>
	{#if label}
		<!-- svelte-ignore a11y_label_has_associated_control -->
		<label use:melt={$labelElement}>{label}</label>
	{/if}
	<button
		use:melt={$trigger}
		class="flex items-center justify-between rounded-md border-2 border-gray-200 px-3 py-2 outline-blue-200"
	>
		{$localSelected?.label || 'Select...'}
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
