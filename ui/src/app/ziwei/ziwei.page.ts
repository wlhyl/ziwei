import { Component, OnInit } from '@angular/core';
import { ZiweistorageService } from '../services/horostorage/ziweistorage.service';
import { ActivatedRoute, Router } from '@angular/router';
import { Title } from '@angular/platform-browser';

@Component({
  selector: 'app-ziwei',
  templateUrl: './ziwei.page.html',
  styleUrls: ['./ziwei.page.scss'],
})
export class ZiweiPage implements OnInit {
  title = '紫微斗数';

  ziweiData = this.storage.ziweiData;

  constructor(
    private storage: ZiweistorageService,
    private router: Router,
    private route: ActivatedRoute,
    private titleService: Title
  ) {}

  ngOnInit() {
    this.titleService.setTitle(this.title);
  }

  getZiwei() {
    this.storage.ziweiData = this.ziweiData;
    this.router.navigate(['pan'], {
      relativeTo: this.route,
    });
  }
}
