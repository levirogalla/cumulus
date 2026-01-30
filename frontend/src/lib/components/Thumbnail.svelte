<script lang="ts">
	import { NO_MEDIA_IMG } from "$lib/constants";
	import { onMount } from "svelte";

  let { key }: { key: string } = $props();
  let thumbnailUrl = $state(NO_MEDIA_IMG);


  onMount(async () => {
    const res = await fetch(`/v1/media/${encodeURI(key)}/thumbnail`);
    console.log(res.url);
		const blob = await res.blob();
    console.log(blob);
		thumbnailUrl = URL.createObjectURL(blob);
  })
</script>

<img src={thumbnailUrl} alt={'No Media'} class="h-48 w-full object-cover" />