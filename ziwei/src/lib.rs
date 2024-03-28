mod star;
mod utils;
// mod ziwei;
mod error;
mod power;
mod vstar;
mod ziwei_star;

pub use error::Error;
pub use power::Power;
pub use star::{Star, StarCategory};
pub use vstar::Vstar;
pub use ziwei_star::StarName;

use star::get_strs;
use utils::{kong_wang, ziwei_day_from_lunar_day, ziwei_month_from_lunar_month};

const HOUSES: [&str; 12] = [
    "命宫", "兄弟", "夫妻", "子女", "财帛", "疾厄", "迁移", "奴仆", "官禄", "田宅", "福德", "相貌",
];

use ganzhiwuxing::{
    DiZhi::{self, *},
    GanZhi, TianGan,
};

use horo_date_time::HoroDateTime;
use lunar_calendar::{lunar_calendar, LunarCalendar};

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct ZiWei {
    /// 出生时间
    native_date: HoroDateTime,
    /// 推运时间
    process_date: HoroDateTime,
    /// 性别，男：true，女：false
    masculine: bool,
    ///  出生时刻的农历
    native_lunar_calendar: LunarCalendar,
    /// 推运时刻的农历
    process_lunar_calendar: LunarCalendar,

    /// 空亡                
    kong: (DiZhi, DiZhi),
    /// 宫干，以寅为起点，顺时针排列
    gong_gan: Vec<TianGan>,
    /// 十二宫
    houses: Vec<String>,
    /// 身宫
    shen_gong: DiZhi,
    /// 五行局
    wu_xing_num: u8,
    /// 命主
    lord: String,
    /// 身主
    shen_lord: String,
    /// 星
    stars: [Vec<Star>; 12],
    /// 大运
    primary_process: DiZhi,
    /// 小限
    profection: String,
    /// 流月
    liu_month: DiZhi,
    /// 流日
    liu_day: DiZhi,
}

impl ZiWei {
    //起盘时间： year, month, day, hour, minute, second
    // 性别： masculine， 男：true
    // 星历表位置：ephePath
    pub fn new(
        native_date: HoroDateTime,
        process_date: HoroDateTime,
        masculine: bool, // 男：true，女：false
        ephe_path: &str,
    ) -> Result<Self, Error> {
        if process_date.jd_utc < native_date.jd_utc {
            return Err(Error::InvalidProcessDateTime(
                "推运时间必需大于等于出生时间".to_string(),
            ));
        }

        // 计算农历
        let native_lunar_calendar = lunar_calendar(
            native_date.year,
            native_date.month,
            native_date.day,
            native_date.hour,
            native_date.minute,
            native_date.second,
            ephe_path,
        )
        .map_err(|error| {
            Error::Function(format!(
                "计算出
        生时间农历错误：{error}"
            ))
        })?;
        let process_lunar_calendar = lunar_calendar(
            process_date.year,
            process_date.month,
            process_date.day,
            process_date.hour,
            process_date.minute,
            process_date.second,
            ephe_path,
        )
        .map_err(|error| Error::Function(format!("计算推运时间农历错误：{error}")))?;

        // 计算空亡
        let kong = kong_wang(&native_lunar_calendar.lunar_day_gan_zhi);

        // 宫干
        let n = native_lunar_calendar.lunar_month_gan_zhi.zhi().minus(&寅) as isize;
        // 寅的天干，计算五行局还要用到
        let yin_gan = native_lunar_calendar.lunar_month_gan_zhi.gan().plus(-n);
        let gong_gan: Vec<_> = (0..12).into_iter().map(|n| yin_gan.plus(n)).collect();

        // 命宫
        let n = native_lunar_calendar.time_gan_zhi.zhi().minus(&子) as isize;
        let ziwei_month_num = ziwei_month_from_lunar_month(&native_lunar_calendar.lunar_month)?;
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
        let shen_gong = ziwei_month_zhi.plus(n);
        // 身主
        let shen_lord = match native_lunar_calendar.lunar_year_gan_zhi.zhi() {
            子 => "火星".to_string(),
            午 => "铃星".to_string(),
            丑 | 未 => "天相".to_string(),
            寅 | 申 => "天梁".to_string(),
            辰 | 戌 => "文昌".to_string(),
            巳 | 亥 => "天机".to_string(),
            卯 | 酉 => "天同".to_string(),
        };

        //寅对应的宫位，此值也用来计算五行局
        let n = ming_gong.minus(&寅) as usize;
        let houses: Vec<_> = (0..12)
            .into_iter()
            .map(|i| HOUSES[(12 + n - i) % 12].to_string()) //宫位逆时钍排列，因此是n-1
            .collect();

        // 五行局
        // gong_gan[n]是命宫天干
        let wu_xing_num = match GanZhi::new(&yin_gan.plus(n as isize), &ming_gong)
            .map_err(|error| Error::Function(error))?
            .na_yin()
        {
            ganzhiwuxing::WuXing::木 => 3u8,
            ganzhiwuxing::WuXing::火 => 6,
            ganzhiwuxing::WuXing::土 => 5,
            ganzhiwuxing::WuXing::金 => 4,
            ganzhiwuxing::WuXing::水 => 2,
        };

        // 大运
        let n = (process_date.year - native_date.year + 1 - wu_xing_num as i32) / 10; // 虚岁，+1
        let n = n as isize;
        let primary_process =
            if masculine == native_lunar_calendar.lunar_year_gan_zhi.gan().masculine() {
                ming_gong.plus(n)
            } else {
                ming_gong.plus(-n)
            };

        let n: usize = primary_process.minus(&寅).into();
        let process_gan = gong_gan[n];

        // 小限
        let profection = match native_lunar_calendar.lunar_year_gan_zhi.zhi() {
            寅 | 午 | 戌 => 辰,
            申 | 子 | 辰 => 戌,
            亥 | 卯 | 未 => 丑,
            巳 | 酉 | 丑 => 未,
        };
        let n = (process_date.year - native_date.year) as isize;
        let profection = if masculine {
            profection.plus(n).to_string()
        } else {
            profection.plus(-n).to_string()
        };

        // 斗君
        let dou_jun = process_lunar_calendar
            .lunar_year_gan_zhi
            .zhi()
            .plus(1 - ziwei_month_num as isize)
            .plus(native_lunar_calendar.time_gan_zhi.zhi().minus(&子).into());

        // 流月
        let process_ziwei_month_num =
            ziwei_month_from_lunar_month(&process_lunar_calendar.lunar_month)?;
        let liu_month = dou_jun.plus((process_ziwei_month_num - 1).into());

        // 流日
        let process_ziwei_day_num = ziwei_day_from_lunar_day(&process_lunar_calendar.lunar_day)?;

        let liu_day = liu_month.plus((process_ziwei_day_num - 1).into());

        // 主星
        let ziwei_day = ziwei_day_from_lunar_day(&native_lunar_calendar.lunar_day)?;
        let stars = get_strs(
            &native_lunar_calendar.lunar_year_gan_zhi,
            wu_xing_num,
            ziwei_month_num,
            ziwei_day,
            &native_lunar_calendar.time_gan_zhi.zhi(),
            &process_gan,
            &process_lunar_calendar.lunar_year_gan_zhi.gan(),
        );

        Ok(Self {
            native_date,
            process_date,
            native_lunar_calendar,
            process_lunar_calendar,
            masculine,

            kong,
            gong_gan,
            houses,
            shen_gong,
            wu_xing_num,
            stars,
            lord,
            shen_lord,
            primary_process,
            profection,
            liu_month,
            liu_day,
        })
    }
}
