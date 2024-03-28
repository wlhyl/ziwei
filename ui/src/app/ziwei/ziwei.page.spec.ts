import { ComponentFixture, TestBed } from '@angular/core/testing';
import { ZiweiPage } from './ziwei.page';

describe('ZiweiPage', () => {
  let component: ZiweiPage;
  let fixture: ComponentFixture<ZiweiPage>;

  beforeEach(async(() => {
    fixture = TestBed.createComponent(ZiweiPage);
    component = fixture.componentInstance;
    fixture.detectChanges();
  }));

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
