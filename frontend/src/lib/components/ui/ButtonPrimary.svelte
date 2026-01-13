<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  type ButtonPrimaryProps = HTMLButtonAttributes & {
    children?: Snippet;
    class?: string;
  };

  let { children, class: className = "", ...rest }: ButtonPrimaryProps = $props();
  let classes = $derived(
    `px-3 py-1 text-xs rounded border bg-blue-600 text-white hover:shadow-md active:bg-blue-700 ${className}`
  );
</script>

<button
  {...rest}
  class={classes}
>
  {@render children?.()}
</button>