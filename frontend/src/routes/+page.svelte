<script lang="ts">
  import { enhance } from "$app/forms";
	import Table from "$lib/components/Table.svelte";
	import type { ColDef } from "ag-grid-community";
  import type { PageData } from "./$types";
	import type { ServerFileObjectMetadata } from "$lib/models";
	import ButtonPrimary from "$lib/components/ui/ButtonPrimary.svelte";
	import ButtonSecondary from "$lib/components/ui/ButtonSecondary.svelte";

  let { data }: { data: PageData } = $props();

  let fileTableCols: ColDef<ServerFileObjectMetadata>[] = [
    { field: "key", headerName: "File Key" },
    { field: "size", headerName: "Size (bytes)" },
    { field: "content_type", headerName: "Content Type" },
    { field: "last_modified", headerName: "Last Modified" },
    { field: "etag", headerName: "ETag" }
  ];

  // Track selected files so we can show feedback in the UI
  let selectedFileNames: string[] = $state([]);

  function handleFilesChange(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    selectedFileNames = input.files
      ? Array.from(input.files).map((file) => file.name)
      : [];
  }
</script>


<main class="p-12">
  <div>

    <h1 class="m-auto font-bold">Home Cloud</h1>
  </div>

  <section class="mb-4">
    <h2>Upload files</h2>

    <div class="mt-2 flex w-full">
 <form
  method="POST"
  action="?/upload"
  use:enhance
  enctype="multipart/form-data"
  class="relative w-full flex flex-col"
>

        <label
          for="files"
          class="flex flex-col items-center justify-center w-full h-32 border-2 border-dashed rounded cursor-pointer text-sm text-gray-500"
        >
          <span class="font-medium">Drag files here or click to select</span>

          {#if selectedFileNames.length < 6 && selectedFileNames.length > 0}
            <p>{selectedFileNames.join(', ')} selected</p>
            <!-- <ul class="mt-1 max-h-16 overflow-y-auto text-xs text-gray-700 w-full px-4">
              {#each selectedFileNames as name}
                <li class="truncate">• {name}</li>
              {/each}
            </ul> -->
          {:else}
            <span class="mt-2 text-xs text-gray-600">
              {selectedFileNames.length} file{selectedFileNames.length === 1 ? "" : "s"} selected
            </span>
          {/if}
        </label>

        <input
          multiple
          type="file"
          name="files"
          id="files"
          class="hidden"
          onchange={handleFilesChange}
        />

    <div class="mt-2 self-end">
    <ButtonPrimary class="" type="submit">
      Submit
    </ButtonPrimary>

    <ButtonSecondary class="" type="button" onclick={() => selectedFileNames = []}>
      clear
    </ButtonSecondary>
    </div>
      </form>
    </div>
  </section>

  <section>
    <Table rowData={data.files} columnDefs={fileTableCols} class="h-72" />
  </section>
</main>