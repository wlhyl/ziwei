import { StarName, Vstar } from '../enum';

export function starToVstar(gan: string, star: StarName): Vstar | null {
  if (gan === '甲') {
    switch (star) {
      case StarName.廉贞:
        return Vstar.禄;
      case StarName.破军:
        return Vstar.权;
      case StarName.武曲:
        return Vstar.科;
      case StarName.太阳:
        return Vstar.忌;
      default:
        return null;
    }
  }

  if (gan === '乙') {
    switch (star) {
      case StarName.天机:
        return Vstar.禄;
      case StarName.天梁:
        return Vstar.权;
      case StarName.紫微:
        return Vstar.科;
      case StarName.太阴:
        return Vstar.忌;
      default:
        return null;
    }
  }
  if (gan === '丙') {
    switch (star) {
      case StarName.天同:
        return Vstar.禄;
      case StarName.天机:
        return Vstar.权;
      case StarName.文昌:
        return Vstar.科;
      case StarName.廉贞:
        return Vstar.忌;
      default:
        return null;
    }
  }
  if (gan === '丁') {
    switch (star) {
      case StarName.太阴:
        return Vstar.禄;
      case StarName.天同:
        return Vstar.权;
      case StarName.天机:
        return Vstar.科;
      case StarName.巨门:
        return Vstar.忌;
      default:
        return null;
    }
  }
  if (gan === '戊') {
    switch (star) {
      case StarName.贪狼:
        return Vstar.禄;
      case StarName.太阴:
        return Vstar.权;
      case StarName.右弼:
        return Vstar.科;
      case StarName.天机:
        return Vstar.忌;
      default:
        return null;
    }
  }
  if (gan === '己') {
    switch (star) {
      case StarName.武曲:
        return Vstar.禄;
      case StarName.贪狼:
        return Vstar.权;
      case StarName.天梁:
        return Vstar.科;
      case StarName.文曲:
        return Vstar.忌;
      default:
        null;
    }
  }
  if (gan === '庚') {
    switch (star) {
      case StarName.太阳:
        return Vstar.禄;
      case StarName.武曲:
        return Vstar.权;
      case StarName.太阴:
        return Vstar.科;
      case StarName.天同:
        return Vstar.忌;
      default:
        return null;
    }
  }
  if (gan === '辛') {
    switch (star) {
      case StarName.巨门:
        return Vstar.禄;
      case StarName.太阳:
        return Vstar.权;
      case StarName.文曲:
        return Vstar.科;
      case StarName.文昌:
        return Vstar.忌;
      default:
        return null;
    }
  }
  if (gan === '壬') {
    switch (star) {
      case StarName.天梁:
        return Vstar.禄;
      case StarName.紫微:
        return Vstar.权;
      case StarName.天府:
        return Vstar.科;
      case StarName.武曲:
        return Vstar.忌;
      default:
        return null;
    }
  }
  if (gan === '癸') {
    switch (star) {
      case StarName.破军:
        return Vstar.禄;
      case StarName.巨门:
        return Vstar.权;
      case StarName.太阴:
        return Vstar.科;
      case StarName.贪狼:
        return Vstar.忌;
      default:
        return null;
    }
  }

  return null;
}
