<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import type { LayoutData } from './$types';
	import Sidebar from '$lib/widgets/Sidebar.svelte';
	import type { Snippet } from 'svelte';

	let { data, children }: { data: LayoutData, children: Snippet } = $props();


</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>



<main class="grid h-screen grid-cols-[200px_auto] grid-rows-1 p-0">
	<Sidebar activeRoute={data.url}/>
	{@render children?.()}	
</main>

