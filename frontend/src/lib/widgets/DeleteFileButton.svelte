<script lang="ts">
	import { enhance } from "$app/forms";
	import ButtonIcon from "$lib/components/ui/ButtonIcon.svelte";
	import ButtonSecondary from "$lib/components/ui/ButtonSecondary.svelte";
	import type { BucketName } from "$lib/models";

  let { key, variant = "text", class: className, bucket = "files"}: { key: string, variant: "text"|"icon", class?: string, bucket?: BucketName} = $props();

  let iconClass = $derived(`m-px ${className}`)
  let textClass = $derived(`${className}`)
</script>

<form action="/files/?/delete&bucket={bucket}" method="post" use:enhance>
  <input type="hidden" name="key" id="key" value="{key}">
  {#if variant == "icon"}
    <ButtonIcon type="submit" iconSrc={"/trash.png"} class={iconClass}></ButtonIcon>
  {:else}
    <ButtonSecondary type="submit" class={textClass}>Delete</ButtonSecondary>
  {/if}
</form>