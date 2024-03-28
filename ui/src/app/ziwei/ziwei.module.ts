import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

import { IonicModule } from '@ionic/angular';

// http访问
// import { HttpClientModule } from '@angular/common/http';

import { ZiweiPageRoutingModule } from './ziwei-routing.module';

import { HoroCommonModule } from '../horo-common/horo-common.module';

import { ZiweiPage } from './ziwei.page';
import { PanComponent } from './pan/pan.component';
import { HouseComponent } from './house/house.component';
import { InfoComponent } from './info/info.component';

@NgModule({
  imports: [
    CommonModule,
    FormsModule,
    IonicModule,
    // HttpClientModule,
    ZiweiPageRoutingModule,
    HoroCommonModule,
  ],
  declarations: [ZiweiPage, PanComponent, HouseComponent, InfoComponent],
})
export class ZiweiPageModule {}
