<script lang="ts">
	import Table from "$lib/components/Table.svelte";
	import ButtonIcon from "$lib/components/ui/ButtonIcon.svelte";
	import ButtonPrimary from "$lib/components/ui/ButtonPrimary.svelte";
	import type { ServerFileObjectMetadata } from "$lib/models";
	import type { ColDef, ICellRendererComp, ICellRendererParams } from "ag-grid-community";
	import { mount } from "svelte";
	import DeleteFileButton from "./DeleteFileButton.svelte";

  let { files }: { files: ServerFileObjectMetadata[] } = $props()

    class TableActionButtons implements ICellRendererComp {
  private eGui!: HTMLElement;
  private component!: ButtonPrimary;

  init(params: ICellRendererParams<any, any, any>) {
    this.eGui = document.createElement('div') as HTMLElement;
    this.eGui.style.display = 'flex';
    this.eGui.style.justifyContent = 'space-between';
    this.eGui.style.alignItems = 'center';
    // this.eGui.style.gap = '0.5rem'; // even spacing between icons

    mount(ButtonIcon, { target: this.eGui as HTMLElement, props: { onclick: ()=>{console.log(params.data.key)}, iconSrc: "/download.png", class: "m-[1px]"}})
    mount(DeleteFileButton, { target: this.eGui as HTMLElement, props: { variant: "icon", key: params.data.key }})
  }

  getGui(): HTMLElement {
    return this.eGui as HTMLElement;
  }

  refresh(params: ICellRendererParams<any, any, any>): boolean {
    // update props if they depend on params
    // this.component.$set({ ... });
    return true;
  }

  destroy(): void {
    // (this.component as ButtonPrimary).$destroy();
  }
}

  let min_flex = 1;
  let width = 70;
  let fileTableCols: ColDef<ServerFileObjectMetadata>[] = [
    { field: "key", headerName: "File Key", flex: min_flex},
    { field: "size", headerName: "Size (bytes)", flex: min_flex},
    // { field: "content_type", headerName: "Content Type", flex: min_flex},
    { field: "last_modified", headerName: "Last Modified", flex: min_flex},
    { field: "etag", headerName: "ETag", flex: min_flex},
    { headerName: "",  filter: false, sortable: false, cellRenderer: TableActionButtons, width: width, minWidth: width, maxWidth: width, resizable: false, suppressSizeToFit: true}
  ];
</script>

{#snippet deleteIcon()}
  delete  
{/snippet}

{#snippet downloadIcon()}
  download
{/snippet}

<Table rowData={files} columnDefs={fileTableCols} class="h-full" />