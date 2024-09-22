<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import Icon from '@iconify/svelte';
	import { melt, createSelect } from '@melt-ui/svelte';
	import type { SelectOption } from './types';
	import { cn } from './utils';

	export let label: string = 'Select an option';
	export let options: SelectOption[];
	export let defaultSelected: SelectOption | undefined = undefined;
	export let className: string = '';

	const dispatch = createEventDispatcher<{ select: SelectOption }>();

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

	$: if ($selected) {
		dispatch('select', $selected);
	}
</script>

<div class={cn('flex flex-col', className)}>
	<!-- svelte-ignore a11y-label-has-associated-control -->
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
