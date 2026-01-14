<script lang="ts">
	import { enhance } from "$app/forms";
	import ButtonIcon from "$lib/components/ui/ButtonIcon.svelte";
	import ButtonSecondary from "$lib/components/ui/ButtonSecondary.svelte";

  let { key, variant = "text", class: className }: { key: string, variant: "text"|"icon", class?: string} = $props();

  let iconClass = $derived(`m-px ${className}`)
  let textClass = $derived(`${className}`)
</script>

<form action="/?/delete" method="post" use:enhance>
  <input type="hidden" name="key" id="key" value="{key}">
  {#if variant == "icon"}
    <ButtonIcon type="submit" iconSrc={"/trash.png"} class={iconClass}></ButtonIcon>
  {:else}
    <ButtonSecondary type="submit" class={textClass}>Delete</ButtonSecondary>
  {/if}
</form>