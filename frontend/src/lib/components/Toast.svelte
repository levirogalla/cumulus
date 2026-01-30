<script lang="ts">
	import type { Snippet } from 'svelte';
	import ButtonX from './ui/ButtonX.svelte';

	const variantColors = {
		hint: 'bg-gray-50 outline-gray-500',
		error: 'bg-red-500 outline-red-800 text-gray-100',
		warning: 'bg-amber-300 outline-amber-900',
		success: ' bg-green-600 outline-green-900 text-gray-100'
	};

	let {
		show = $bindable(),
		variant = 'hint',
		children
	}: {
		show: boolean;
		variant?: 'hint' | 'error' | 'warning' | 'success';
		children?: Snippet;
	} = $props();
	let classes = $derived(
		`px-3 fixed flex flex-row bottom-3 right-3 w-80 h-15 rounded-sm outline shadow-xl ${variantColors[variant]}`
	);
</script>

{#if show}
	<div class={classes + " align-middle justify-between"}>
		<ButtonX onclick={()=>show=false} class=""></ButtonX>
    <div class="flex flex-col justify-center w-full items-center">
      {@render children?.()}
    </div>
	</div>
{/if}
