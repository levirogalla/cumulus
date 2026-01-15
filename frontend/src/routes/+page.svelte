<script lang="ts">
	import { enhance } from '$app/forms';
	import Table from '$lib/components/Table.svelte';
	import type {
		AgPromise,
		ColDef,
		ICellRendererComp,
		ICellRendererParams
	} from 'ag-grid-community';
	import type { PageData } from './$types';
	import type { ServerFileObjectMetadata } from '$lib/models';
	import ButtonPrimary from '$lib/components/ui/ButtonPrimary.svelte';
	import ButtonSecondary from '$lib/components/ui/ButtonSecondary.svelte';
	import { invalidateAll } from '$app/navigation';
	import { mount } from 'svelte';
	import FileTable from '$lib/widgets/FileTable.svelte';
	import Sidebar from '$lib/widgets/Sidebar.svelte';
	import ButtonIcon from '$lib/components/ui/ButtonIcon.svelte';
	import Modal from '$lib/components/Modal.svelte';

	let { data }: { data: PageData } = $props();

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
						action="?/upload"
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

						<div class="self-end mt-4 flex gap-2">
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
<main class="grid h-screen grid-cols-[200px_auto] grid-rows-1 p-0">
	<Sidebar />
	<div class="grid h-full grid-rows-[50px_auto]">
		<div class="border border-black bg-gray-50">
			<section class="h-full flex flex-row justify-between">
          <div></div>
          <ButtonIcon onclick={() => (showUploadFileModal = true)} class="p-0 m-1" iconSrc="/upload.png" height={6} width={6}/>
					

			</section>
		</div>
		<div>
			<section class="h-full">
				<FileTable files={data.files} />
			</section>
		</div>
	</div>
</main>
