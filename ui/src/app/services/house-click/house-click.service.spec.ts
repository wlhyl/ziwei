import { TestBed } from '@angular/core/testing';

import { HouseClickService } from './house-click.service';

describe('HouseClickService', () => {
  let service: HouseClickService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(HouseClickService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
