import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { ZiWei } from 'src/app/interface/response';
import { ApiService } from 'src/app/services/api/api.service';
import { ZiweistorageService } from 'src/app/services/horostorage/ziweistorage.service';

@Component({
  selector: 'app-pan',
  templateUrl: './pan.component.html',
  styleUrls: ['./pan.component.scss'],
})
export class PanComponent implements OnInit {
  title = '紫微斗数';

  ziweiData = this.storage.ziweiData;

  pan: ZiWei | null = null;

  width = '800px';
  height = '800px';

  isAlertOpen = false;
  alertButtons = ['OK'];
  message = '';

  loading = false;

  constructor(
    private api: ApiService,
    private storage: ZiweistorageService,
    private titleService: Title
  ) {}

  ngOnInit() {
    this.titleService.setTitle(this.title);
    this.getZiweiPan();
  }

  getZiweiPan() {
    this.loading = true;
    this.api.getZiwei(this.ziweiData).subscribe({
      next: (response) => (this.pan = response),
      error: (error) => {
        const message = error.message + ' ' + error.error.message;
        this.message = message;
        this.isAlertOpen = true;
      },
      complete: () => (this.loading = false),
    });
  }

  changeStep(step: {
    year: number;
    month: number;
    day: number;
    hour: number;
    minute: number;
    second: number;
  }) {
    let date = new Date(
      this.ziweiData.process_date.year,
      this.ziweiData.process_date.month - 1,
      this.ziweiData.process_date.day,
      this.ziweiData.process_date.hour,
      this.ziweiData.process_date.minute,
      this.ziweiData.process_date.second
    );

    date.setFullYear(date.getFullYear() + step.year);
    date.setMonth(date.getMonth() + step.month);
    date.setDate(date.getDate() + step.day);
    date.setHours(date.getHours() + step.hour);
    date.setMinutes(date.getMinutes() + step.minute);
    date.setSeconds(date.getSeconds() + step.second);

    this.ziweiData.process_date.year = date.getFullYear();
    this.ziweiData.process_date.month = date.getMonth() + 1;
    this.ziweiData.process_date.day = date.getDate();
    this.ziweiData.process_date.hour = date.getHours();
    this.ziweiData.process_date.minute = date.getMinutes();
    this.ziweiData.process_date.second = date.getSeconds();

    this.getZiweiPan();
  }
}
