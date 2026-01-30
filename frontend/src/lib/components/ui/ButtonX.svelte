<script lang="ts">
	import type { Snippet } from "svelte";
	import type { HTMLButtonAttributes } from "svelte/elements";

  let { class: className, ...rest }: { class?: string } & HTMLButtonAttributes = $props();
  

  let classes = $derived(`rounded-md p-1 text-2xl leading-none  ${className}`)
</script>

<button
	type="button"
	class={classes}
	aria-label="Close"
  {...rest}
>
	×
</button>
