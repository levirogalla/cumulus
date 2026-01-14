import {
  AllCommunityModule,
  ModuleRegistry,
  createGrid,
  type GridOptions
} from 'ag-grid-community';

ModuleRegistry.registerModules([AllCommunityModule]);

export { createGrid, type GridOptions };

// example of how to create buttons in the table
//   class TableActionButtons implements ICellRendererComp {
//   eGui: HTMLElement | unknown;
//   component: ButtonPrimary | unknown;

//   init(params: ICellRendererParams<any, any, any>) {
//     this.eGui = document.createElement('div');
//     mount(ButtonPrimary, { target: this.eGui as HTMLElement, props: { onclick: ()=>{console.log(params.data.key)}, children: text
//     }})
//   }

//   getGui(): HTMLElement {
//     return this.eGui as HTMLElement;
//   }

//   refresh(params: ICellRendererParams<any, any, any>): boolean {
//     // update props if they depend on params
//     // this.component.$set({ ... });
//     return true;
//   }

//   destroy(): void {
//     // (this.component as ButtonPrimary).$destroy();
//   }
// }