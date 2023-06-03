export interface RequestData {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  masculine: boolean;
  // 推运年，最小值1900
  process_year: number;
  // 推运月
  process_month: number;
  // 推运日
  process_day: number;
  // 推运时
  process_hour: number;
  // 推运分
  process_minute: number;
  // 推运秒
  process_second: number;
}

export interface ZiWeiPan {
  lunar_calendar: {
    year: string;
    month: string;
    day: string;
    time_gan_zhi: string;
  };
  solar_term_first: {
    name: string;
    year: number;
    month: number;
    day: number;
    hour: number;
    minute: number;
    second: number;
  };
  solar_term_second: {
    name: string;
    year: number;
    month: number;
    day: number;
    hour: number;
    minute: number;
    second: number;
  };
  si_zhu: {
    year: string;
    month: string;
    day: string;
    time: string;
  };
  kong: Array<string>;
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
  second_process: string;
  // 斗君
  dou_jun: string;
}

export interface Star {
  // 星名
  name: string;
  // 星类别
  category: string;
  // 五行
  wu_xing: string|null;
  // 所管宫主
  lord: string|null,
  // 化曜
  v_star: string | null;
  // 庙、旺、得、利 、平、不、陷
  power: string | null;
}

// #[derive(Serialize, ToSchema)]
// pub enum StarCategory {
//   /// 北斗
//   N,
//   /// 南斗
//   S,
//   /// 中天
//   M,
// }
