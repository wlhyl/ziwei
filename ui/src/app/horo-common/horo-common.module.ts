import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { IonicModule } from '@ionic/angular';

// 数据双向绑定
import { FormsModule } from '@angular/forms';

import { DateTimeComponent } from './date-time/date-time.component';

@NgModule({
  declarations: [DateTimeComponent],
  imports: [CommonModule, IonicModule, FormsModule],
  exports: [FormsModule, DateTimeComponent],
})
export class HoroCommonModule {}
