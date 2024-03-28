import { Power, StarCategory, StarName, Vstar } from '../enum';

export interface ZiWei {
  // 出生时间
  native_date: HoroDateTime;
  // 推运时间
  process_date: HoroDateTime;
  //  出生时刻的农历
  native_lunar_calendar: LunarCalendar;
  // 推运时刻的农历
  process_lunar_calendar: LunarCalendar;
  // 性别，男:true，女:false
  masculine: boolean;

  // 空亡
  kong: Array<string>;
  // 宫干，以寅为起点，顺时针排列
  gong_gan: Array<string>;
  // 十二宫
  houses: Array<string>;
  // 身宫
  shen_gong: string;
  // 五行局
  wu_xing_num: number;
  // 命主
  lord: string;
  // 身主
  shen_lord: string;
  // 星
  stars: Array<Array<Star>>;
  // 大运
  primary_process: string;
  // 小限
  profection: string;
  // 流月
  liu_month: string;
  // 流日
  liu_day: string;
}

export interface HoroDateTime {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  tz: number; //时区
}

export interface LunarCalendar {
  // 闰年:true
  is_lean_year: boolean;

  // 农历年，干支表示
  lunar_year: string; // GanZhi,

  // 农历月，以正月、二月、......、十月、冬月、腊月表示
  lunar_month: string;

  // 农历日，以初一、初二、……、二十九、三十表示
  lunar_day: string;

  // 农历年干支，按节气换年
  lunar_year_gan_zhi: string; // GanZhi,

  // 农历月干支，按节气换月
  lunar_month_gan_zhi: string; // GanZhi,

  // 日干支
  lunar_day_gan_zhi: string; // GanZhi,

  // 时干支
  time_gan_zhi: string; // GanZhi,

  // 节
  solar_term_first: SolarTerm;

  // 中气
  solar_term_second: SolarTerm;
}

// 节气
export interface SolarTerm {
  name: string;
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
}

export interface Star {
  // 星名
  name: StarName;
  // 星类别
  category: StarCategory;
  // 五行
  wu_xing: Array<string>;
  // 化曜
  v_star: Vstar | null;
  // 庙、旺、得、利 、平、不、陷
  power: Power | null;
  // 大运四化
  process_v_star: Vstar | null;
  // 流年四化
  process_year_v_star: Vstar | null;
  // 描述
  describe: string | null;
}
