import { Component, Input, OnInit } from '@angular/core';
import { ZiWei } from 'src/app/interface/response';

@Component({
  selector: 'app-info',
  templateUrl: './info.component.html',
  styleUrls: ['./info.component.scss'],
})
export class InfoComponent implements OnInit {
  @Input()
  pan: ZiWei | null = null;

  get sex(): string | null {
    if (this.pan === null) return null;
    let s0 = this.pan.masculine ? '男' : '女';

    let s = ['甲', '丙', '戊', '庚', '壬'].includes(
      this.pan.native_lunar_calendar.lunar_year[0]
    )
      ? '阳'
      : '阴';
    return `${s}${s0}`;
  }

  // 五行局
  get wuXingNum(): string | null {
    if (this.pan === null) return null;
    switch (this.pan.wu_xing_num) {
      case 2:
        return '水二局';
      case 3:
        return '木三局';
      case 4:
        return '金四局';
      case 5:
        return '土五局';
      default:
        return '火六局';
    }
  }

  constructor() {}

  ngOnInit() {}
}
