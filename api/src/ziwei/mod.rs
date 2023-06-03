use ganzhiwuxing::{DiZhi::*, GanZhi, TianGan::*};
use lunar_calendar::lunar_calendar;
use serde::Serialize;
use utoipa::ToSchema;

use self::{
    star::{get_strs, Star},
    utils::{ziwei_day_from_lunar_day, ziwei_month_from_lunar_month},
};

pub mod star;
pub mod ziwei_star;
pub mod utils;

const HOUSES: [&str; 12] = [
    "命宫", "兄弟", "夫妻", "子女", "财帛", "疾厄", "迁移", "奴仆", "官禄", "田宅", "福德", "相貌",
];

#[derive(Debug, Serialize, ToSchema)]
pub struct LunarCalendar {
    /// 农历年
    year: String,
    /// 农历月
    month: String,
    /// 农历日
    day: String,
    /// 农历时
    time_gan_zhi: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SolarTerm {
    name: String,
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

#[derive(Serialize, ToSchema)]
pub struct SiZhu {
    year: String,
    month: String,
    day: String,
    time: String,
}

fn kong_wang(day: &GanZhi) -> (String, String) {
    let gan = &day.gan();
    let zhi = &day.zhi();

    let delta: isize = gan.minus(&甲).into();

    let xun_shou = zhi.plus(-delta);

    (xun_shou.plus(-2).to_string(), xun_shou.plus(-1).to_string())
}

#[derive(Serialize, ToSchema)]
pub struct ZiWeiPan {
    /// 农历日期
    lunar_calendar: LunarCalendar,
    /// 节
    solar_term_first: SolarTerm,
    /// 气
    solar_term_second: SolarTerm,
    /// 四柱  
    si_zhu: SiZhu,
    /// 空亡                
    kong: (String, String),
    /// 宫干，以寅为起点，顺时针排列
    gong_gan: Vec<String>,
    /// 十二宫
    houses: Vec<String>,
    /// 身宫
    shen_gong: String,
    /// 五行局
    wu_xing_num: u8,
    /// 命主
    lord: String,
    /// 身主
    shen_lord: String,
    /// 星
    stars: [Vec<Star>; 12],
    /// 大运
    primary_process: String,
    /// 小限
    second_process: String,
    /// 斗君
    dou_jun: String,
}

impl ZiWeiPan {
    //起盘时间： year, month, day, hour, minute, second
    // 性别： masculine， 男：true
    // 星历表位置：ephePath
    pub fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        masculine: bool,
        process_year: i32,
        process_month: u8,
        process_day: u8,
        process_hour: u8,
        process_minute: u8,
        process_second: u8,
        ephe_path: &str,
    ) -> Result<ZiWeiPan, String> {
        // 将公历转换成农历
        let lunar_calendar_t = lunar_calendar(year, month, day, hour, minute, second, ephe_path)?;
        let process_lunar_calendar_t = lunar_calendar(
            process_year,
            process_month,
            process_day,
            process_hour,
            process_minute,
            process_second,
            ephe_path,
        )?;

        let lunar_calendar_struct = LunarCalendar {
            year: lunar_calendar_t.lunar_year.to_string(),
            month: lunar_calendar_t.lunar_month.clone(),
            day: lunar_calendar_t.lunar_day.clone(),
            time_gan_zhi: lunar_calendar_t.time_gan_zhi.to_string(),
        };

        let solar_term_first = SolarTerm {
            name: lunar_calendar_t.solar_term_first.name,
            year: lunar_calendar_t.solar_term_first.year,
            month: lunar_calendar_t.solar_term_first.month,
            day: lunar_calendar_t.solar_term_first.day,
            hour: lunar_calendar_t.solar_term_first.hour,
            minute: lunar_calendar_t.solar_term_first.minute,
            second: lunar_calendar_t.solar_term_first.second,
        };

        let solar_term_second = SolarTerm {
            name: lunar_calendar_t.solar_term_second.name,
            year: lunar_calendar_t.solar_term_second.year,
            month: lunar_calendar_t.solar_term_second.month,
            day: lunar_calendar_t.solar_term_second.day,
            hour: lunar_calendar_t.solar_term_second.hour,
            minute: lunar_calendar_t.solar_term_second.minute,
            second: lunar_calendar_t.solar_term_second.second,
        };

        let si_zhu = SiZhu {
            year: lunar_calendar_t.lunar_year_gan_zhi.to_string(),
            month: lunar_calendar_t.lunar_month_gan_zhi.to_string(),
            day: lunar_calendar_t.lunar_day_gan_zhi.to_string(),
            time: lunar_calendar_t.time_gan_zhi.to_string(),
        };

        // 计算空亡
        let kong = kong_wang(&lunar_calendar_t.lunar_day_gan_zhi);

        // 宫干
        let n = lunar_calendar_t.lunar_month_gan_zhi.zhi().minus(&寅) as isize;
        let yin_gan = lunar_calendar_t.lunar_month_gan_zhi.gan().plus(-n); // 寅的天干，计算五行局还要用到
        let gong_gan: Vec<_> = (0..12)
            .into_iter()
            .map(|n| yin_gan.plus(n).to_string())
            .collect();

        // 命宫
        let n = lunar_calendar_t.time_gan_zhi.zhi().minus(&子) as isize;
        let ziwei_month_num = ziwei_month_from_lunar_month(&lunar_calendar_t.lunar_month)?;
        let ziwei_month_zhi = 寅.plus(ziwei_month_num as isize - 1);
        let ming_gong = ziwei_month_zhi.plus(-n);
        // 命主
        let lord = match ming_gong {
            子 => "贪狼".to_string(),
            亥 | 丑 => "巨门".to_string(),
            寅 | 戌 => "禄存".to_string(),
            卯 | 酉 => "文曲".to_string(),
            午 => "破军".to_string(),
            申 | 辰 => "廉贞".to_string(),
            巳 | 未 => "武曲".to_string(),
        };
        // 身宫
        let shen_gong = ziwei_month_zhi.plus(n).to_string();
        // 身主
        let shen_lord = match lunar_calendar_t.lunar_year_gan_zhi.zhi() {
            子 => "火星".to_string(),
            午 => "铃星".to_string(),
            丑 | 未 => "天相".to_string(),
            寅 | 申 => "天梁".to_string(),
            辰 | 戌 => "文昌".to_string(),
            巳 | 亥 => "天机".to_string(),
            卯 | 酉 => "天同".to_string(),
        };

        let n = ming_gong.minus(&寅) as usize; //寅对应的宫位，此值也用来计算五行局
        let houses: Vec<_> = (0..12)
            .into_iter()
            .map(|i| HOUSES[(12 + n - i) % 12].to_string()) //宫位逆时钍排列，因此是n-1
            .collect();

        // 五行局
        // gong_gan[n]是命宫天干
        let wu_xing_num = match GanZhi::new(&yin_gan.plus(n as isize), &ming_gong)?.na_yin() {
            ganzhiwuxing::WuXing::木 => 3u8,
            ganzhiwuxing::WuXing::火 => 6,
            ganzhiwuxing::WuXing::土 => 5,
            ganzhiwuxing::WuXing::金 => 4,
            ganzhiwuxing::WuXing::水 => 2,
        };

        // 大运
        let n = (process_year - year + 1 - wu_xing_num as i32) / 10; // 虚岁，+1
        let n = n as isize;
        let primary_process = if masculine == lunar_calendar_t.lunar_year_gan_zhi.gan().masculine()
        {
            ming_gong.plus(n).to_string()
        } else {
            ming_gong.plus(-n).to_string()
        };

        // 小限
        let second_process = match lunar_calendar_t.lunar_year_gan_zhi.zhi() {
            寅 | 午 | 戌 => 辰,
            申 | 子 | 辰 => 戌,
            亥 | 卯 | 未 => 丑,
            巳 | 酉 | 丑 => 未,
        };
        let n = (process_year - year) as isize;
        let second_process = if masculine {
            second_process.plus(n).to_string()
        } else {
            second_process.plus(-n).to_string()
        };

        // 斗君
        let dou_jun = process_lunar_calendar_t
            .lunar_year_gan_zhi
            .zhi()
            .plus(1 - ziwei_month_num as isize)
            .plus(lunar_calendar_t.time_gan_zhi.zhi().minus(&子).into())
            .to_string();

        // 主星
        let ziwei_day = ziwei_day_from_lunar_day(&lunar_calendar_t.lunar_day)?;
        let stars = get_strs(
            &lunar_calendar_t.lunar_year_gan_zhi,
            wu_xing_num,
            ziwei_month_num,
            ziwei_day,
            &lunar_calendar_t.time_gan_zhi.zhi(),
        );

        Ok(Self {
            lunar_calendar: lunar_calendar_struct,
            solar_term_first,
            solar_term_second,
            si_zhu,
            kong,
            gong_gan,
            houses,
            shen_gong,
            wu_xing_num,
            stars,
            lord,
            shen_lord,
            primary_process,
            second_process,
            dou_jun,
        })
    }
}
