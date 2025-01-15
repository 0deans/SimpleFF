<script lang="ts">
	import Icon from '@iconify/svelte';
	import { createCheckbox, melt } from '@melt-ui/svelte';
	import { cn } from './utils';

	interface Props {
		label: string;
		checked: boolean | 'indeterminate';
		className?: string;
	}

	let { label, checked = $bindable(), className }: Props = $props();

	const {
		elements: { root, input },
		states: { checked: localChecked },
		helpers: { isChecked, isIndeterminate }
	} = createCheckbox();

	$effect(() => localChecked.set(checked));
	$effect(() => {
		checked = $localChecked;
	});
</script>

<div class={cn('flex items-center', className)}>
	<button
		use:melt={$root}
		class="flex size-7 appearance-none items-center justify-center rounded-lg border border-gray-300 bg-white text-blue-600 hover:opacity-75"
		id="checkbox"
	>
		{#if $isIndeterminate}
			<Icon icon="ic:round-minus" class="size-5" />
		{:else if $isChecked}
			<Icon icon="material-symbols:check-rounded" class="size-5" />
		{/if}
		<input use:melt={$input} />
	</button>
	<label class="pl-2 font-medium" for="checkbox">
		{label}
	</label>
</div>
