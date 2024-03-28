import { Injectable } from '@angular/core';
import { StarName, Vstar } from 'src/app/enum';

@Injectable({
  providedIn: 'root',
})
export class HouseClickService {
  gan = '';
  house: string | null = null;

  constructor() {}
}
