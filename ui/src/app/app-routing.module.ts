import { NgModule } from '@angular/core';
import { PreloadAllModules, RouterModule, Routes } from '@angular/router';

const routes: Routes = [
  {
    path: 'ziwei',
    loadChildren: () =>
      import('./ziwei/ziwei.module').then((m) => m.ZiweiPageModule),
  },
  {
    path: '**',
    redirectTo: 'ziwei',
    // pathMatch: 'full'
  },
];

@NgModule({
  imports: [
    RouterModule.forRoot(routes, { preloadingStrategy: PreloadAllModules }),
  ],
  exports: [RouterModule],
})
export class AppRoutingModule {}
