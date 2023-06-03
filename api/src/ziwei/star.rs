use ganzhiwuxing::{
    DiZhi::{self, *},
    GanZhi,
    TianGan::*,
};
use serde::Serialize;
use utoipa::ToSchema;

use super::ziwei_star::{
    Power, Vstar,
    ZiWeiStar::{self, *},
};

#[derive(Serialize, ToSchema)]
pub struct Star {
    /// 星名
    pub name: ZiWeiStar,
    /// 星类别
    pub category: StarCategory,
    /// 五行
    wu_xing: Option<String>,
    /// 所管宫主
    lord: Option<String>,
    /// 化曜
    v_star: Option<Vstar>,
    /// 庙、旺、得、利 、平、不、陷
    power: Option<Power>,
}

#[derive(Serialize, ToSchema)]
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
) -> [Vec<Star>; 12] {
    let mut stars: [Vec<Star>; 12] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
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
    stars[k as usize].push(Star {
        name: 紫微,
        category: StarCategory::N,
        wu_xing: 紫微.wu_xing(),
        v_star: 紫微.v_star(&year.gan()),
        power: 紫微.power(&寅.plus(k.into())),
        lord: 紫微.lord(),
    });

    // 天机
    let k = (k + 12 - 1) % 12;
    stars[k as usize].push(Star {
        name: 天机,
        category: StarCategory::N,
        wu_xing: 天机.wu_xing(),
        v_star: 天机.v_star(&year.gan()),
        power: 天机.power(&寅.plus(k.into())),
        lord: 天机.lord(),
    });

    // 太阳
    let k = (k + 12 - 2) % 12;
    stars[k as usize].push(Star {
        name: 太阳,
        category: StarCategory::N,
        wu_xing: 太阳.wu_xing(),
        v_star: 太阳.v_star(&year.gan()),
        power: 太阳.power(&寅.plus(k.into())),
        lord: 太阳.lord(),
    });

    // 武曲
    let k = (k + 12 - 1) % 12;
    stars[k as usize].push(Star {
        name: 武曲,
        category: StarCategory::N,
        wu_xing: 武曲.wu_xing(),
        v_star: 武曲.v_star(&year.gan()),
        power: 武曲.power(&寅.plus(k.into())),
        lord: 武曲.lord(),
    });

    // 天同
    let k = (k + 12 - 1) % 12;
    stars[k as usize].push(Star {
        name: 天同,
        category: StarCategory::N,
        wu_xing: 天同.wu_xing(),
        v_star: 天同.v_star(&year.gan()),
        power: 天同.power(&寅.plus(k.into())),
        lord: 天同.lord(),
    });

    // 廉贞
    let k = (k + 12 - 3) % 12;
    stars[k as usize].push(Star {
        name: 廉贞,
        category: StarCategory::N,
        wu_xing: 廉贞.wu_xing(),
        v_star: 廉贞.v_star(&year.gan()),
        power: 廉贞.power(&寅.plus(k.into())),
        lord: 廉贞.lord(),
    });

    // 紫微+天府=12
    // 天府
    let k = (k + 12 - 4) % 12; // 重新得到紫微的位置
    let k = (12 - k) % 12; //k=-0，则12-k=0，因此需要除以12取余
    stars[k as usize].push(Star {
        name: 天府,
        category: StarCategory::S,
        wu_xing: 天府.wu_xing(),
        v_star: 天府.v_star(&year.gan()),
        power: 天府.power(&寅.plus(k.into())),
        lord: 天府.lord(),
    });

    // 太阴
    let k = (k + 1) % 12;
    stars[k as usize].push(Star {
        name: 太阴,
        category: StarCategory::S,
        wu_xing: 太阴.wu_xing(),
        v_star: 太阴.v_star(&year.gan()),
        power: 太阴.power(&寅.plus(k.into())),
        lord: 太阴.lord(),
    });

    // 贪狼
    let k = (k + 1) % 12;
    stars[k as usize].push(Star {
        name: 贪狼,
        category: StarCategory::S,
        wu_xing: 贪狼.wu_xing(),
        v_star: 贪狼.v_star(&year.gan()),
        power: 贪狼.power(&寅.plus(k.into())),
        lord: 贪狼.lord(),
    });

    // 巨门
    let k = (k + 1) % 12;
    stars[k as usize].push(Star {
        name: 巨门,
        category: StarCategory::S,
        wu_xing: 巨门.wu_xing(),
        v_star: 巨门.v_star(&year.gan()),
        power: 巨门.power(&寅.plus(k.into())),
        lord: 巨门.lord(),
    });

    // 天相
    let k = (k + 1) % 12;
    stars[k as usize].push(Star {
        name: 天相,
        category: StarCategory::S,
        wu_xing: 天相.wu_xing(),
        v_star: None,
        power: 天相.power(&寅.plus(k.into())),
        lord: 天相.lord(),
    });

    // 天梁
    let k = (k + 1) % 12;
    stars[k as usize].push(Star {
        name: 天梁,
        category: StarCategory::S,
        wu_xing: 天梁.wu_xing(),
        v_star: 天梁.v_star(&year.gan()),
        power: 天梁.power(&寅.plus(k.into())),
        lord: 天梁.lord(),
    });

    // 七杀
    let k = (k + 1) % 12;
    stars[k as usize].push(Star {
        name: 七杀,
        category: StarCategory::S,
        wu_xing: 七杀.wu_xing(),
        v_star: None,
        power: 七杀.power(&寅.plus(k.into())),
        lord: 七杀.lord(),
    });

    // 破军
    let k = (k + 4) % 12;
    stars[k as usize].push(Star {
        name: 破军,
        category: StarCategory::S,
        wu_xing: 破军.wu_xing(),
        v_star: 破军.v_star(&year.gan()),
        power: 破军.power(&寅.plus(k.into())),
        lord: 破军.lord(),
    });

    // 左辅
    // 寅=1
    // 辰在数组中下标是：2
    // 辰+(ziwei_month-寅)
    // 左辅=2+(ziwei_month-1)
    let k = (1 + ziwei_month) % 12;
    stars[k as usize].push(Star {
        name: 左辅,
        category: StarCategory::Benefic,
        wu_xing: 左辅.wu_xing(),
        v_star: None,
        power: 左辅.power(&寅.plus(k.into())),
        lord: 左辅.lord(),
    });

    // 右弼
    // 寅=1
    // 戌在数组中下标是：8
    // 戌-(ziwei_month-寅)
    // 右弼=8-(ziwei_month-1)=9-ziwei_month
    let k = (21 - ziwei_month) % 12;
    stars[k as usize].push(Star {
        name: 右弼,
        category: StarCategory::Benefic,
        wu_xing: 右弼.wu_xing(),
        v_star: 右弼.v_star(&year.gan()),
        power: 右弼.power(&寅.plus(k.into())),
        lord: 右弼.lord(),
    });

    // 文曲
    // 辰在数组中下标是：2
    // 辰+(time_zhi-子)
    // 文曲=2+(time_zhi-1)
    let k = (2 + time_zhi.minus(&子)) % 12;
    stars[k as usize].push(Star {
        name: 文曲,
        category: StarCategory::Benefic,
        wu_xing: 文曲.wu_xing(),
        v_star: 文曲.v_star(&year.gan()),
        power: 文曲.power(&寅.plus(k.into())),
        lord: 文曲.lord(),
    });

    // 文昌
    // 戌在数组中下标是：8
    // 戌-(time_zhi-子)
    // 右辅=8-(time_zhi-子)
    let k = (20 - time_zhi.minus(&子)) % 12;
    stars[k as usize].push(Star {
        name: 文昌,
        category: StarCategory::Benefic,
        wu_xing: 文昌.wu_xing(),
        v_star: 文昌.v_star(&year.gan()),
        power: 文昌.power(&寅.plus(k.into())),
        lord: 文昌.lord(),
    });

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
    stars[k as usize].push(Star {
        name: 天魁,
        category: StarCategory::Benefic,
        wu_xing: 天魁.wu_xing(),
        v_star: None,
        power: 天魁.power(&寅.plus(k as isize)),
        lord: 天魁.lord(),
    });
    //天铖
    let k = n.1;
    stars[k as usize].push(Star {
        name: 天铖,
        category: StarCategory::Benefic,
        wu_xing: 天铖.wu_xing(),
        v_star: None,
        power: 天铖.power(&寅.plus(k as isize)),
        lord: 天铖.lord(),
    });

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
    stars[k as usize].push(Star {
        name: 禄存,
        category: StarCategory::Money,
        wu_xing: 禄存.wu_xing(),
        v_star: None,
        power: 禄存.power(&寅.plus(k)),
        lord: 禄存.lord(),
    });
    // 擎羊
    stars[((k + 1) % 12) as usize].push(Star {
        name: 擎羊,
        category: StarCategory::Malefics,
        wu_xing: 擎羊.wu_xing(),
        v_star: None,
        power: 擎羊.power(&寅.plus((k + 1) % 12)),
        lord: 擎羊.lord(),
    });
    // 陀罗
    stars[((k + 12 - 1) % 12) as usize].push(Star {
        name: 陀罗,
        category: StarCategory::Malefics,
        wu_xing: 陀罗.wu_xing(),
        v_star: None,
        power: 陀罗.power(&寅.plus((k + 12 - 1) % 12)),
        lord: 陀罗.lord(),
    });

    // 天马
    let yi_ma = if year.zhi().minus(&寅) % 3 == 0 {
        year.zhi().plus(6)
    } else if year.zhi().plus(4).minus(&寅) % 3 == 0 {
        year.zhi().plus(4).plus(6)
    } else {
        year.zhi().plus(8).plus(6)
    };
    let k = yi_ma.minus(&寅);
    stars[k as usize].push(Star {
        name: 天马,
        category: StarCategory::Money,
        wu_xing: 天马.wu_xing(),
        v_star: None,
        power: 天马.power(&寅.plus(k.into())),
        lord: 天马.lord(),
    });

    // 火星
    let start = match yi_ma.plus(6) {
        寅 => 丑,
        巳 => 卯,
        申 => 寅,
        _ => 酉,
    };
    let k = start.plus(time_zhi.minus(&子).into()).minus(&寅);
    stars[k as usize].push(Star {
        name: 火星,
        category: StarCategory::Malefics,
        wu_xing: 火星.wu_xing(),
        v_star: None,
        power: 火星.power(&寅.plus(k.into())),
        lord: 火星.lord(),
    });

    // 铃星
    let start = match yi_ma.plus(6) {
        寅 => 卯,
        _ => 戌,
    };
    let k = start.plus(time_zhi.minus(&子).into()).minus(&寅);
    stars[k as usize].push(Star {
        name: 铃星,
        category: StarCategory::Malefics,
        wu_xing: 铃星.wu_xing(),
        v_star: None,
        power: 铃星.power(&寅.plus(k.into())),
        lord: 铃星.lord(),
    });

    // 地空
    // 亥-(时-子)
    let k = 亥.plus(子.minus(time_zhi).into()).minus(&寅);
    stars[k as usize].push(Star {
        name: 地空,
        category: StarCategory::Malefics,
        wu_xing: 地空.wu_xing(),
        v_star: None,
        power: 地空.power(&寅.plus(k.into())),
        lord: 地空.lord(),
    });
    // 地劫
    // 亥+(时-子)
    let k = 亥.plus(time_zhi.minus(&子).into()).minus(&寅);
    stars[k as usize].push(Star {
        name: 地劫,
        category: StarCategory::Malefics,
        wu_xing: 地劫.wu_xing(),
        v_star: None,
        power: 地劫.power(&寅.plus(k.into())),
        lord: 地劫.lord(),
    });

    // 红鸾
    // 卯-(年-子)
    let k = 卯.plus(子.minus(&year.zhi()).into()).minus(&寅);
    stars[k as usize].push(Star {
        name: 红鸾,
        category: StarCategory::Common,
        wu_xing: 红鸾.wu_xing(),
        v_star: None,
        power: 红鸾.power(&寅.plus(k.into())),
        lord: 红鸾.lord(),
    });

    // 天喜
    // 酉-(年-子)
    let k = 酉.plus(子.minus(&year.zhi()).into()).minus(&寅);
    stars[k as usize].push(Star {
        name: 天喜,
        category: StarCategory::Common,
        wu_xing: 天喜.wu_xing(),
        v_star: None,
        power: 天喜.power(&寅.plus(k.into())),
        lord: 天喜.lord(),
    });

    // 天姚
    // 丑下标11
    // 正月：1
    // 丑+(月-正月)=11+(month-1)=10-month
    let k = (22 - ziwei_month) % 12;
    stars[k as usize].push(Star {
        name: 天姚,
        category: StarCategory::Common,
        wu_xing: 天姚.wu_xing(),
        v_star: None,
        power: 天姚.power(&寅.plus(k.into())),
        lord: 天姚.lord(),
    });

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
    stars[k as usize].push(Star {
        name: 咸池,
        category: StarCategory::Common,
        wu_xing: 咸池.wu_xing(),
        v_star: None,
        power: 咸池.power(&寅.plus(k as isize)),
        lord: 咸池.lord(),
    });

    // 天刑
    // 酉下标7
    // 正月：1
    // 酉+(月-正月)=7+(month-1)=6-month
    let k = (18 - ziwei_month) % 12;
    stars[k as usize].push(Star {
        name: 天刑,
        category: StarCategory::Common,
        wu_xing: 天刑.wu_xing(),
        v_star: None,
        power: 天刑.power(&寅.plus(k.into())),
        lord: 天刑.lord(),
    });

    stars
}
