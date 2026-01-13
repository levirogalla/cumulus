import {
  AllCommunityModule,
  ModuleRegistry,
  createGrid,
  type GridOptions
} from 'ag-grid-community';

ModuleRegistry.registerModules([AllCommunityModule]);

export { createGrid, type GridOptions };