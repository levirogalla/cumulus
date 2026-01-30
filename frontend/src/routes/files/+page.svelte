<script lang="ts">
	import { enhance } from '$app/forms';
	import ButtonPrimary from '$lib/components/ui/ButtonPrimary.svelte';
	import ButtonSecondary from '$lib/components/ui/ButtonSecondary.svelte';
	import { goto, invalidateAll } from '$app/navigation';
	import { mount } from 'svelte';
	import FileTable from '$lib/widgets/FileTable.svelte';
	import Sidebar from '$lib/widgets/Sidebar.svelte';
	import ButtonIcon from '$lib/components/ui/ButtonIcon.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import type { PageData, PageProps, PageServerData } from './$types';
	import type { BucketName } from '$lib/models';
	import { navigating } from '$app/state';


	let { data, form }: PageProps = $props();

	const getOtherBucketName = () => {
		if (data.bucket == 'files') {
			return 'media';
		} else if (data.bucket == 'media') {
			return 'files';
		} else {
			console.error("unrecognized bucket name, defaulting to 'files'");
			return 'files';
		}
	};

	// Track selected files so we can show feedback in the UI
	let selectedFileNames: string[] = $state([]);

	function handleFilesChange(event: Event) {
		const input = event.currentTarget as HTMLInputElement;
		selectedFileNames = input.files ? Array.from(input.files).map((file) => file.name) : [];
	}

	let showUploadFileModal = $state(false);
</script>

<Modal bind:show={showUploadFileModal} title="Upload">
	<form
		method="POST"
		action="?/upload&bucket={data.bucket}"
		use:enhance
		enctype="multipart/form-data"
		class="relative flex w-full flex-col"
	>
		<label
			for="files"
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

		<input multiple hidden type="file" name="files" id="files" onchange={handleFilesChange} />

		<div class="mt-4 flex gap-2 self-end">
			<ButtonSecondary class="" type="button" onclick={() => (selectedFileNames = [])}>
				clear
			</ButtonSecondary>
			<ButtonPrimary
				class=""
				type="submit"
				onclick={async () => {
					await invalidateAll();
					showUploadFileModal = false;
					selectedFileNames = [];
				}}
			>
				Submit
			</ButtonPrimary>
		</div>
	</form>
</Modal>


<div class="grid h-full grid-rows-[50px_auto]">
	<div class="border-b border-black bg-gray-50">
		<section class="flex h-full flex-row items-center justify-between px-3">
			<div></div>
			<div>
				<ButtonSecondary onclick={async () => await goto(`/files?bucket=${getOtherBucketName()}`)}
					>Show {getOtherBucketName()[0].toUpperCase() + getOtherBucketName().slice(1)}</ButtonSecondary
				>
				<ButtonPrimary onclick={() => (showUploadFileModal = true)}>Upload Files</ButtonPrimary>
			</div>
		</section>
	</div>
	<div>
		<section class="h-full">
			<FileTable files={data.files} bucket={data.bucket} />
		</section>
	</div>
</div>
