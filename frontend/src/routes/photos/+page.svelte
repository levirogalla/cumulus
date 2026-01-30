<script lang="ts">
	import { enhance } from '$app/forms';
	import { invalidateAll } from '$app/navigation';
	import { onMount, tick } from 'svelte';
	import Modal from '$lib/components/Modal.svelte';
	import ButtonPrimary from '$lib/components/ui/ButtonPrimary.svelte';
	import ButtonSecondary from '$lib/components/ui/ButtonSecondary.svelte';
	import type { PageData } from './$types';
	import Toast from '$lib/components/Toast.svelte';
	import Darken from '$lib/components/ui/Darken.svelte';
	import Thumbnail from '$lib/components/Thumbnail.svelte';
	import { NO_MEDIA_IMG } from '$lib/constants';

	let { data }: { data: PageData } = $props();

	let selectedFileNames: string[] = $state([]);
	let showUploadFileModal: boolean = $state(false);

	let formLoading = $state(false);
	let showSuccessToast = $state(false)

	
	let showMediaDetailsModal: boolean = $state(false);
	let mediaUrl: string = $state(NO_MEDIA_IMG);

	function handleFilesChange(event: Event) {
		const input = event.currentTarget as HTMLInputElement;
		selectedFileNames = input.files ? Array.from(input.files).map((file) => file.name) : [];
	}

	let scroller: HTMLDivElement | null = null;

	onMount(async () => {
		await tick();
		scroller?.scrollTo({ top: scroller.scrollHeight, behavior: 'instant' as ScrollBehavior });
	});

	const showMedia = async (key: string) => {
		const res = await fetch(`/v1/media/${encodeURI(key)}`);
		const blob = await res.blob();
		console.log(blob);
		mediaUrl = URL.createObjectURL(blob);
		showMediaDetailsModal = true;
	};
</script>

<Modal bind:show={showUploadFileModal} title="Upload">
	<form
		method="POST"
		action="?/upload"
		use:enhance={() => {
			formLoading = true;
			showUploadFileModal = false;
			selectedFileNames = [];
			return async ({ update }) => {
				formLoading = false;
				showSuccessToast = true;
				update();
			};
		}}
		enctype="multipart/form-data"
		class="relative flex w-full flex-col"
	>
		<label
			for="medias"
			class="flex h-32 w-full cursor-pointer flex-col items-center justify-center rounded border border-dashed text-sm text-gray-500"
		>
			<span class="font-medium">Drag files here or click to select</span>

			{#if selectedFileNames.length < 6 && selectedFileNames.length > 0}
				<p>{selectedFileNames.join(', ')} selected</p>
			{:else}
				<span class="mt-2 text-xs text-gray-600">
					{selectedFileNames.length} file{selectedFileNames.length === 1 ? '' : 's'} selected
				</span>
			{/if}
		</label>

		<input multiple hidden type="file" name="medias" id="medias" onchange={handleFilesChange} />

		<div class="mt-4 flex gap-2 self-end">
			<ButtonSecondary class="" type="button" onclick={() => (selectedFileNames = [])}>
				clear
			</ButtonSecondary>
			<ButtonPrimary class="" type="submit">Submit</ButtonPrimary>
		</div>
	</form>
</Modal>

<Modal bind:show={showMediaDetailsModal} title="Media Details">
	<img src={mediaUrl} alt="" />
</Modal>

<Darken show={formLoading}></Darken>

<Toast show={showSuccessToast} variant="success">
	Upload Successful!
</Toast>

<div class="grid h-full min-h-0 grid-rows-[50px_1fr]">
	<div class="border-b border-black bg-gray-50">
		<section class="flex h-full flex-row items-center justify-between px-3">
			<div></div>
			<!-- <ButtonIcon
					onclick={() => (showUploadFileModal = true)}
					class="m-1 p-0 h-7 w-8"
					iconSrc="/upload.png"

					height={6}
					width={6}
				/> -->
			<ButtonPrimary onclick={() => (showUploadFileModal = true)}>Upload Media</ButtonPrimary>
		</section>
	</div>
	<div class="min-h-0 overflow-hidden">
		<section class="h-full min-h-0">
			<div bind:this={scroller} class="h-full min-h-0 overflow-x-hidden overflow-y-auto p-4">
				<div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
					{#each data.medias as media}
						<button
							class="overflow-hidden rounded shadow"
							onclick={async () => await showMedia(media.key)}
						>
							<Thumbnail key={media.key}></Thumbnail>
							
						</button>
					{/each}
				</div>
			</div>
		</section>
	</div>
</div>
