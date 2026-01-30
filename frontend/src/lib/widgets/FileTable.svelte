<script lang="ts">
	import Table from '$lib/components/Table.svelte';
	import ButtonIcon from '$lib/components/ui/ButtonIcon.svelte';
	import ButtonPrimary from '$lib/components/ui/ButtonPrimary.svelte';
	import type { BucketName, ServerFileObjectMetadata } from '$lib/models';
	import type { ColDef, ICellRendererComp, ICellRendererParams } from 'ag-grid-community';
	import { mount } from 'svelte';
	import DeleteFileButton from './DeleteFileButton.svelte';

	let { files, bucket="files" }: { files: ServerFileObjectMetadata[]; bucket?: BucketName } = $props();

	class TableActionButtons implements ICellRendererComp {
		private eGui!: HTMLElement;
		private component!: ButtonPrimary;

		init(params: ICellRendererParams<any, any, any>) {
			this.eGui = document.createElement('div') as HTMLElement;
			this.eGui.style.display = 'flex';
			this.eGui.style.justifyContent = 'space-between';
			this.eGui.style.alignItems = 'start';
			// this.eGui.style.gap = '0.5rem'; // even spacing between icons

			mount(ButtonIcon, {
				target: this.eGui as HTMLElement,
				props: {
					onclick: () => {
						console.log(params.data.key);
					},
					iconSrc: '/download.png',
					class: 'h-full w-6'
				}
			});
			mount(DeleteFileButton, {
				target: this.eGui as HTMLElement,
				props: { variant: 'icon', bucket: bucket, key: params.data.key, class: 'h-full w-6' }
			});
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
		{ field: 'key', headerName: 'File Key', flex: min_flex },
		{ field: 'size', headerName: 'Size (bytes)', flex: min_flex },
		// { field: "content_type", headerName: "Content Type", flex: min_flex},
		{ field: 'last_modified', headerName: 'Last Modified', flex: min_flex },
		{ field: 'etag', headerName: 'ETag', flex: min_flex },
		{
			headerName: '',
			filter: false,
			sortable: false,
			cellRenderer: TableActionButtons,
			width: width,
			minWidth: width,
			maxWidth: width,
			resizable: false,
			suppressSizeToFit: true
		}
	];
</script>

<Table rowData={files} columnDefs={fileTableCols} class="h-full" />
