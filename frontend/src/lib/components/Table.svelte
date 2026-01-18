<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import type { ColDef, GridApi, GridOptions } from "ag-grid-community";
  import { createGrid } from "$lib/agGrid";
  import { TABLE_THEME } from "$lib/constants";

  type Row = any;

  let { rowData, columnDefs, class: className }: {
    rowData: Row[];
    columnDefs: ColDef<Row>[];
    class?: string;
  } = $props();

  let gridElement: HTMLDivElement | null = null;
  let grid: GridApi | null = null;

  onMount(() => {
    const gridOptions: GridOptions<Row> = {
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

    grid = createGrid(gridElement!, gridOptions);
  });

  $effect(() => {
    if (!grid) return;
    grid.setGridOption("rowData", rowData);
  });

  $effect(() => {
    if (!grid) return;
    grid.setGridOption("columnDefs", columnDefs);
  });

  onDestroy(() => {
    grid?.destroy();
    grid = null;
  });

  let classes = $derived(className ?? "");
</script>


<div bind:this={gridElement} id="myGrid" class="{classes}"></div>