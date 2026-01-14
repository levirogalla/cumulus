<script lang="ts">
  import { enhance } from "$app/forms";
	import Table from "$lib/components/Table.svelte";
	import type { AgPromise, ColDef, ICellRendererComp, ICellRendererParams } from "ag-grid-community";
  import type { PageData } from "./$types";
	import type { ServerFileObjectMetadata } from "$lib/models";
	import ButtonPrimary from "$lib/components/ui/ButtonPrimary.svelte";
	import ButtonSecondary from "$lib/components/ui/ButtonSecondary.svelte";
	import { invalidateAll } from "$app/navigation";
	import { mount } from "svelte";
	import FileTable from "$lib/widgets/FileTable.svelte";

  

  let { data }: { data: PageData } = $props();



  // Track selected files so we can show feedback in the UI
  let selectedFileNames: string[] = $state([]);

  function handleFilesChange(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    selectedFileNames = input.files
      ? Array.from(input.files).map((file) => file.name)
      : [];
  }

</script>

{#snippet text()}
  hi
{/snippet}


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
          class="flex flex-col items-center justify-center w-full h-32 border border-dashed rounded cursor-pointer text-sm text-gray-500"
        >
          <span class="font-medium">Drag files here or click to select</span>

          {#if selectedFileNames.length < 6 && selectedFileNames.length > 0}
            <p>{selectedFileNames.join(', ')} selected</p>

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
    <ButtonPrimary class="" type="submit" onclick={async () => {
      await invalidateAll();
      selectedFileNames = []}}>
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
    <FileTable files={data.files} />
  </section>
</main>