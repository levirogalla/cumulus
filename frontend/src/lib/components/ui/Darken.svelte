<script lang="ts">
	import type { Snippet } from 'svelte';
  import type { HTMLAttributes } from "svelte/elements";

	let {
		children,
		show = $bindable(),
		class: className,
    tabindex = 0,
    ...rest
	}: HTMLAttributes<HTMLDivElement> & {
		children?: Snippet;
		show: boolean;
		class?: string;
    tabindex?: number
  } = $props();

</script>

{#if show}
	<div
		class="fixed inset-0 z-50 grid place-items-center bg-black/50 p-4"
		role="button"
    tabindex={tabindex}
    {...rest}
	>
		{@render children?.()}
	</div>
{/if}
