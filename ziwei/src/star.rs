use ganzhiwuxing::{
    DiZhi::{self, *},
    GanZhi,
    TianGan::{self, *},
    WuXing,
};
#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

use crate::power::Power;
use crate::vstar::Vstar;

use super::ziwei_star::StarName::{self, *};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct Star {
    /// 星名
    pub name: StarName,
    /// 星类别
    pub category: StarCategory,
    /// 五行
    wu_xing: Vec<WuXing>,
    /// 化曜
    v_star: Option<Vstar>,
    /// 庙、旺、得、利 、平、不、陷
    power: Option<Power>,
    /// 大运四化
    process_v_star: Option<Vstar>,
    // 流年四化
    process_year_v_star: Option<Vstar>,
    /// 描述
    describe: Option<String>,
}

impl Star {
    fn new(
        name: StarName,
        category: StarCategory,
        native_year_gan: &TianGan,
        house: &DiZhi,
        process_gan: &TianGan,
        process_year_gan: &TianGan,
    ) -> Self {
        Self {
            name,
            category,
            wu_xing: name.wu_xing(),
            v_star: name.v_star(native_year_gan),
            power: name.power(house),
            process_v_star: name.v_star(&process_gan),
            process_year_v_star: name.v_star(&process_year_gan),
            describe: name.describe(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub enum StarCategory {
    /// 北斗
    N,
    /// 南斗
    S,
    /// 吉星
    Benefic,
    /// 凶星
    Malefics,
    /// 天禄、天马
    Money,
    /// 普通星
    Common,
}

pub fn get_strs(
    year: &GanZhi,
    wu_xing_num: u8,
    ziwei_month: u8,
    ziwei_day: u8,
    time_zhi: &DiZhi,
    // 大运宫干
    process_gan: &TianGan,
    // 流年天干
    process_year_gan: &TianGan,
) -> [Vec<Star>; 12] {
    let mut stars: [Vec<Star>; 12] = [
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    // 紫微
    // 算法：日数=k*五行局数+r
    // r==0，取商
    // r!=0,日数+m=k*五行局数+r+(五行局数-r)
    // 日数+m = (k+1)*五行局数
    // m = 五行局-r
    // m是奇数 k+1-m
    // m是偶数:k+1+m
    let r = ziwei_day % wu_xing_num;
    let k = ziwei_day / wu_xing_num;
    let k = if r != 0 {
        let m = wu_xing_num - r;
        if m % 2 == 0 {
            k + 1 + m
        } else {
            k + 1 - m
        }
    } else {
        k
    };
    // k有可能为0
    let k = if k == 0 { 12 } else { k };
    let k = (k - 1) % 12;
    stars[k as usize].push(Star::new(
        紫微,
        StarCategory::N,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 天机
    let k = (k + 12 - 1) % 12;
    stars[k as usize].push(Star::new(
        天机,
        StarCategory::N,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 太阳
    let k = (k + 12 - 2) % 12;
    stars[k as usize].push(Star::new(
        太阳,
        StarCategory::N,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 武曲
    let k = (k + 12 - 1) % 12;
    stars[k as usize].push(Star::new(
        武曲,
        StarCategory::N,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 天同
    let k = (k + 12 - 1) % 12;
    stars[k as usize].push(Star::new(
        天同,
        StarCategory::N,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 廉贞
    let k = (k + 12 - 3) % 12;
    stars[k as usize].push(Star::new(
        廉贞,
        StarCategory::N,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 紫微+天府=12
    // 天府
    let k = (k + 12 - 4) % 12; // 重新得到紫微的位置
    let k = (12 - k) % 12; //k=-0，则12-k=0，因此需要除以12取余
    stars[k as usize].push(Star::new(
        天府,
        StarCategory::S,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 太阴
    let k = (k + 1) % 12;
    stars[k as usize].push(Star::new(
        太阴,
        StarCategory::S,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 贪狼
    let k = (k + 1) % 12;
    stars[k as usize].push(Star::new(
        贪狼,
        StarCategory::S,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 巨门
    let k = (k + 1) % 12;
    stars[k as usize].push(Star::new(
        巨门,
        StarCategory::S,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 天相
    let k = (k + 1) % 12;
    stars[k as usize].push(Star::new(
        天相,
        StarCategory::S,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 天梁
    let k = (k + 1) % 12;
    stars[k as usize].push(Star::new(
        天梁,
        StarCategory::S,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 七杀
    let k = (k + 1) % 12;
    stars[k as usize].push(Star::new(
        七杀,
        StarCategory::S,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 破军
    let k = (k + 4) % 12;
    stars[k as usize].push(Star::new(
        破军,
        StarCategory::S,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 左辅
    // 寅=1
    // 辰在数组中下标是：2
    // 辰+(ziwei_month-寅)
    // 左辅=2+(ziwei_month-1)
    let k = (1 + ziwei_month) % 12;
    stars[k as usize].push(Star::new(
        左辅,
        StarCategory::Benefic,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 右弼
    // 寅=1
    // 戌在数组中下标是：8
    // 戌-(ziwei_month-寅)
    // 右弼=8-(ziwei_month-1)=9-ziwei_month
    let k = (21 - ziwei_month) % 12;
    stars[k as usize].push(Star::new(
        右弼,
        StarCategory::Benefic,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 文曲
    // 辰在数组中下标是：2
    // 辰+(time_zhi-子)
    // 文曲=2+(time_zhi-1)
    let k = (2 + time_zhi.minus(&子)) % 12;
    stars[k as usize].push(Star::new(
        文曲,
        StarCategory::Benefic,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 文昌
    // 戌在数组中下标是：8
    // 戌-(time_zhi-子)
    // 右辅=8-(time_zhi-子)
    let k = (20 - time_zhi.minus(&子)) % 12;
    stars[k as usize].push(Star::new(
        文昌,
        StarCategory::Benefic,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 天魁、天铖
    let n = match year.gan() {
        甲 | 戊 | 庚 => (11, 5), //丑:11，未:5
        乙 | 己 => (10, 6),      // 子:10，申6
        丙 | 丁 => (9, 7),       //亥：9，酉：7
        辛 => (0, 4),            //寅：0，午：4
        壬 | 癸 => (1, 3),       //卯：1，巳：3
    };

    //天魁
    let k = n.0;
    stars[k as usize].push(Star::new(
        天魁,
        StarCategory::Benefic,
        &year.gan(),
        &寅.plus(k as isize),
        &process_gan,
        &process_year_gan,
    ));
    //天铖
    let k = n.1;
    stars[k as usize].push(Star::new(
        天铖,
        StarCategory::Benefic,
        &year.gan(),
        &寅.plus(k as isize),
        &process_gan,
        &process_year_gan,
    ));

    // 禄存
    let k = match year.gan() {
        甲 => 0,
        乙 => 1,
        丙 | 戊 => 3,
        丁 | 己 => 4,
        庚 => 6,
        辛 => 7,
        壬 => 9,
        癸 => 10,
    };
    stars[k as usize].push(Star::new(
        禄存,
        StarCategory::Money,
        &year.gan(),
        &寅.plus(k),
        &process_gan,
        &process_year_gan,
    ));
    // 擎羊
    stars[((k + 1) % 12) as usize].push(Star::new(
        擎羊,
        StarCategory::Malefics,
        &year.gan(),
        &寅.plus((k + 1) % 12),
        &process_gan,
        &process_year_gan,
    ));
    // 陀罗
    stars[((k + 12 - 1) % 12) as usize].push(Star::new(
        陀罗,
        StarCategory::Malefics,
        &year.gan(),
        &寅.plus((k + 12 - 1) % 12),
        &process_gan,
        &process_year_gan,
    ));

    // 天马
    let yi_ma = if year.zhi().minus(&寅) % 3 == 0 {
        year.zhi().plus(6)
    } else if year.zhi().plus(4).minus(&寅) % 3 == 0 {
        year.zhi().plus(4).plus(6)
    } else {
        year.zhi().plus(8).plus(6)
    };
    let k = yi_ma.minus(&寅);
    stars[k as usize].push(Star::new(
        天马,
        StarCategory::Money,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 火星
    let start = match yi_ma.plus(6) {
        寅 => 丑,
        巳 => 卯,
        申 => 寅,
        _ => 酉,
    };
    let k = start.plus(time_zhi.minus(&子).into()).minus(&寅);
    stars[k as usize].push(Star::new(
        火星,
        StarCategory::Malefics,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 铃星
    let start = match yi_ma.plus(6) {
        寅 => 卯,
        _ => 戌,
    };
    let k = start.plus(time_zhi.minus(&子).into()).minus(&寅);
    stars[k as usize].push(Star::new(
        铃星,
        StarCategory::Malefics,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 地空
    // 亥-(时-子)
    let k = 亥.plus(子.minus(time_zhi).into()).minus(&寅);
    stars[k as usize].push(Star::new(
        地空,
        StarCategory::Malefics,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));
    // 地劫
    // 亥+(时-子)
    let k = 亥.plus(time_zhi.minus(&子).into()).minus(&寅);
    stars[k as usize].push(Star::new(
        地劫,
        StarCategory::Malefics,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 红鸾
    // 卯-(年-子)
    let k = 卯.plus(子.minus(&year.zhi()).into()).minus(&寅);
    stars[k as usize].push(Star::new(
        红鸾,
        StarCategory::Common,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 天喜
    // 酉-(年-子)
    let k = 酉.plus(子.minus(&year.zhi()).into()).minus(&寅);
    stars[k as usize].push(Star::new(
        天喜,
        StarCategory::Common,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 天姚
    // 丑下标11
    // 正月：1
    // 丑+(月-正月)=11+(month-1)=10-month
    let k = (22 - ziwei_month) % 12;
    stars[k as usize].push(Star::new(
        天姚,
        StarCategory::Common,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    // 咸池
    // 酉下标11
    // 正月：1
    // 丑+(月-正月)=11+(month-1)=10-month
    let k = match year.zhi() {
        寅 | 午 | 戌 => 1,  // 卯
        申 | 子 | 辰 => 7,  // 酉
        巳 | 酉 | 丑 => 4,  // 午
        亥 | 卯 | 未 => 10, // 子
    };
    stars[k as usize].push(Star::new(
        咸池,
        StarCategory::Common,
        &year.gan(),
        &寅.plus(k as isize),
        &process_gan,
        &process_year_gan,
    ));

    // 天刑
    // 酉下标7
    // 正月：1
    // 酉+(月-正月)=7+(month-1)=6-month
    let k = (18 - ziwei_month) % 12;
    stars[k as usize].push(Star::new(
        天刑,
        StarCategory::Common,
        &year.gan(),
        &寅.plus(k.into()),
        &process_gan,
        &process_year_gan,
    ));

    stars
}
