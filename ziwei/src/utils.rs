use crate::Error;
use ganzhiwuxing::{DiZhi, GanZhi, TianGan::甲};

pub(crate) fn ziwei_month_from_lunar_month(month: &str) -> Result<u8, Error> {
    match month {
        "正月" => Ok(1),
        "二月" => Ok(2),
        "三月" => Ok(3),
        "四月" => Ok(4),
        "五月" => Ok(5),
        "六月" => Ok(6),
        "七月" => Ok(7),
        "八月" => Ok(8),
        "九月" => Ok(9),
        "十月" => Ok(10),
        "冬月" => Ok(11),
        "腊月" => Ok(12),
        "闰正月" => Ok(2),
        "闰二月" => Ok(3),
        "闰三月" => Ok(4),
        "闰四月" => Ok(5),
        "闰五月" => Ok(6),
        "闰六月" => Ok(7),
        "闰七月" => Ok(8),
        "闰八月" => Ok(9),
        "闰九月" => Ok(10),
        "闰十月" => Ok(11),
        "闰冬月" => Ok(12),
        "闰腊月" => Ok(1),
        _ => Err(Error::Function(format!("没有如此月份：{}", month))),
    }
}

pub(crate) fn ziwei_day_from_lunar_day(month: &str) -> Result<u8, Error> {
    match month {
        "初一" => Ok(1),
        "初二" => Ok(2),
        "初三" => Ok(3),
        "初四" => Ok(4),
        "初五" => Ok(5),
        "初六" => Ok(6),
        "初七" => Ok(7),
        "初八" => Ok(8),
        "初九" => Ok(9),
        "初十" => Ok(10),
        "十一" => Ok(11),
        "十二" => Ok(12),
        "十三" => Ok(13),
        "十四" => Ok(14),
        "十五" => Ok(15),
        "十六" => Ok(16),
        "十七" => Ok(17),
        "十八" => Ok(18),
        "十九" => Ok(19),
        "二十" => Ok(20),
        "廿一" => Ok(21),
        "廿二" => Ok(22),
        "廿三" => Ok(23),
        "廿四" => Ok(24),
        "廿五" => Ok(25),
        "廿六" => Ok(26),
        "廿七" => Ok(27),
        "廿八" => Ok(28),
        "廿九" => Ok(29),
        "三十" => Ok(30),
        _ => Err(Error::Function(format!("没有如此日：{}", month))),
    }
}

/// 计算空亡
pub(crate) fn kong_wang(day: &GanZhi) -> (DiZhi, DiZhi) {
    let gan = &day.gan();
    let zhi = &day.zhi();

    let delta: isize = gan.minus(&甲).into();

    let xun_shou = zhi.plus(-delta);

    (xun_shou.plus(-2), xun_shou.plus(-1))
}
