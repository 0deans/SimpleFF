<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLInputAttributes } from 'svelte/elements';
	import { createLabel, melt } from '@melt-ui/svelte';
	import { cn } from './utils';

	interface Props extends HTMLInputAttributes {
		label?: string;
		value: number | string;
		children?: Snippet;
		classNames?: {
			container?: string;
			label?: string;
			input?: string;
		};
	}

	let {
		type = 'text',
		label,
		value = $bindable(),
		id,
		children,
		classNames = {},
		...restInputProps
	}: Props = $props();

	const {
		elements: { root: labelElement }
	} = createLabel();
</script>

<div class={cn('flex w-full flex-col', classNames.container)}>
	{#if label}
		<label use:melt={$labelElement} for={id} class={classNames.label}>{label}</label>
	{/if}
	<input
		bind:value
		{type}
		{id}
		{...restInputProps}
		class={cn('w-full rounded-md border-2 border-gray-200 p-2 outline-blue-200', classNames.input)}
	/>
	{#if children}
		{@render children()}
	{/if}
</div>
