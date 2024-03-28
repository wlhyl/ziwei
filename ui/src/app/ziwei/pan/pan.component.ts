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

  // style = {
  //   pan: {
  width = '800px';
  height = '800px';
  //   },
  // };

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
}
