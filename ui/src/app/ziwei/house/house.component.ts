import { Component, Input } from '@angular/core';
import { StarName, Vstar } from 'src/app/enum';
import { Star } from 'src/app/interface/response';
import { HouseClickService } from 'src/app/services/house-click/house-click.service';
import { starToVstar } from 'src/app/utils/vstar';

@Component({
  selector: 'app-house',
  templateUrl: './house.component.html',
  styleUrls: ['./house.component.scss'],
})
export class HouseComponent {
  @Input()
  name = '';
  @Input()
  gan = '';
  @Input()
  zhi = '';

  @Input()
  stars: Array<Star> = [];

  describe = '';
  isShowDescrible = false;


  constructor(private houseClickService: HouseClickService) {}

  showDescribe(index: number) {
    if (index >= this.stars.length) {
      this.isShowDescrible = false;
      this.describe = '';
      return;
    }

    const describe = this.stars[index].describe;
    if (describe === null) return;

    this.describe = `${this.stars[index].name}：`;
    if (this.stars[index].wu_xing.length != 0)
      this.describe = `${this.describe}五行属${this.stars[index].wu_xing}，`;
    this.describe = `${this.describe}${describe}`;
    this.isShowDescrible = true;
  }

  closeDescribe() {
    this.describe = '';
    this.isShowDescrible = false;
  }

  houseClick() {
    if (this.houseClickService.house === this.zhi) {
      this.houseClickService.house = null;
      return;
    }

    this.houseClickService.house = this.zhi;
    this.houseClickService.gan = this.gan;
  }

  vStar(star: StarName): Vstar | null {
    if (this.houseClickService.house === null) return null;

    return starToVstar(this.houseClickService.gan, star);
  }
}
