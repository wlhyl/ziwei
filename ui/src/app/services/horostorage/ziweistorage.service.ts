import { Injectable } from '@angular/core';
import { ZiWeiRenReust } from 'src/app/interface/request-data';

@Injectable({
  providedIn: 'root',
})
export class ZiweistorageService {
  public set ziweiData(date: ZiWeiRenReust) {
    localStorage.setItem('ziwei_data', JSON.stringify(date));
  }

  public get ziweiData(): ZiWeiRenReust {
    let j = localStorage.getItem('ziwei_data');
    if (j) {
      return JSON.parse(j) as ZiWeiRenReust;
    }

    let t = this.nowDate();

    return {
      native_date: {
        year: t.year,
        month: t.month,
        day: t.day,
        hour: t.hour,
        minute: t.minute,
        second: t.second,
      },

      process_date: {
        year: t.year,
        month: t.month,
        day: t.day,
        hour: t.hour,
        minute: t.minute,
        second: t.second,
      },

      masculine: true,
      describe: '',
    };
  }

  constructor() {}

  private nowDate(): date {
    let t = new Date();
    let year = t.getFullYear();
    let month = t.getMonth() + 1;
    let day = t.getDate();
    let hour = t.getHours();
    let minute = t.getMinutes();
    let second = t.getSeconds();

    let st = false;

    // 判断夏令时
    let d1 = new Date(year, 1, 1);
    let tz = d1.getTimezoneOffset() / -60;
    // let d2 = new Date(this.horo.year,7,1);
    if (t.getTimezoneOffset() != d1.getTimezoneOffset()) {
      st = true;
      tz -= 1;
    }

    return {
      year,
      month,
      day,
      hour,
      minute,
      second,
      tz,
      st,
    };
  }
}

interface date {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  tz: number;
  st: boolean;
}
