<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  type ButtonIconProps = HTMLButtonAttributes & {
    /** Optional text/label content */
    children?: Snippet;
    /** Optional snippet for an inline SVG/icon component */
    icon?: Snippet;
    /** Path or data URL for svg/png/etc */
    iconSrc?: string;
    height?: number,
    width?: number,
    /** Alt text for the img when using iconSrc */
    iconAlt?: string;
    class?: string;
  };

  let {
    children,
    icon,
    iconSrc,
    height = 4,
    width = 4,
    iconAlt = "",
    class: className = "",
    ...rest
  }: ButtonIconProps = $props();

  let classes = $derived(
    `inline-flex items-center p-1 text-xs bg-transparent text-gray-700 hover:outline outline-gray-300 active:bg-gray-200 ${className}`
  );

  let imgClasses = $derived(`h-${height} w-${width}`)
</script>

<button
  {...rest}
  class={classes}
>
  {#if icon}
    {@render icon?.()}
  {:else if iconSrc}
    <img src={iconSrc} alt={iconAlt} class={imgClasses} />
  {/if}

  {#if children}
    {@render children?.()}
  {/if}
</button>