<script lang='ts'>
   import { onMount } from 'svelte';
  import type { ColDef, GridOptions, GridApi } from 'ag-grid-community';
  import { createGrid } from '$lib/agGrid';
  import { TABLE_THEME } from '$lib/constants';

  type Row = any;

  let { rowData, columnDefs, class: className }: {
    rowData: Row[];
    columnDefs: ColDef<Row>[];
    class?: string;
  } = $props();


  let gridElement: HTMLElement | unknown = null;
  onMount(() => {
    const gridOptions: GridOptions<unknown> = {
    rowData,
    columnDefs,
    theme: TABLE_THEME,

    defaultColDef: {
      sortable: true,
      filter: true,
      resizable: true,
      flex: 1,
      minWidth: 100
    }
  };
    const grid: GridApi = createGrid(gridElement as HTMLElement, gridOptions);

        // whenever rowData changes, push it into AG Grid
  $effect(() => {
    if (grid) {
      grid.setGridOption('rowData', rowData);
    }
  });
  })

  let classes = $derived(`${className}`);


</script>

<svelte:head>
   <script src="https://cdn.jsdelivr.net/npm/ag-grid-community/dist/ag-grid-community.min.js"></script>
</svelte:head>

<div bind:this={gridElement} id="myGrid" class="{classes}"></div>