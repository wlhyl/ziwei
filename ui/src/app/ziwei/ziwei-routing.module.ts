import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { ZiweiPage } from './ziwei.page';

import { PanComponent } from './pan/pan.component';

const routes: Routes = [
  {
    path: '',
    component: ZiweiPage,
  },
  {
    path: 'pan',
    component: PanComponent,
  },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class ZiweiPageRoutingModule {}
