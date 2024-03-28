use ganzhiwuxing::{
    DiZhi::{self, *},
    TianGan,
    WuXing::{self, *},
};

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

use ganzhiwuxing::TianGan::*;

use crate::power::Power::{self, *};
use crate::vstar::Vstar::{self, *};

use StarName::*;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Clone, Copy)]
pub enum StarName {
    紫微,
    天机,
    太阳,
    武曲,
    天同,
    廉贞,
    天府,
    太阴,
    贪狼,
    巨门,
    天相,
    天梁,
    七杀,
    破军,
    左辅,
    右弼,
    文昌,
    文曲,
    天魁,
    天铖,
    禄存,
    擎羊,
    陀罗,
    天马,
    火星,
    铃星,
    地空,
    地劫,
    红鸾,
    天喜,
    天姚,
    咸池,
    天刑,
}

impl StarName {
    // 化曜,virtual star
    pub fn v_star(&self, gan: &TianGan) -> Option<Vstar> {
        match gan {
            甲 => match self {
                廉贞 => Some(禄),
                破军 => Some(权),
                武曲 => Some(科),
                太阳 => Some(忌),
                _ => None,
            },
            乙 => match self {
                天机 => Some(禄),
                天梁 => Some(权),
                紫微 => Some(科),
                太阴 => Some(忌),
                _ => None,
            },
            丙 => match self {
                天同 => Some(禄),
                天机 => Some(权),
                文昌 => Some(科),
                廉贞 => Some(忌),
                _ => None,
            },
            丁 => match self {
                太阴 => Some(禄),
                天同 => Some(权),
                天机 => Some(科),
                巨门 => Some(忌),
                _ => None,
            },
            戊 => match self {
                贪狼 => Some(禄),
                太阴 => Some(权),
                右弼 => Some(科),
                天机 => Some(忌),
                _ => None,
            },
            己 => match self {
                武曲 => Some(禄),
                贪狼 => Some(权),
                天梁 => Some(科),
                文曲 => Some(忌),
                _ => None,
            },
            庚 => match self {
                太阳 => Some(禄),
                武曲 => Some(权),
                太阴 => Some(科),
                天同 => Some(忌),
                _ => None,
            },
            辛 => match self {
                巨门 => Some(禄),
                太阳 => Some(权),
                文曲 => Some(科),
                文昌 => Some(忌),
                _ => None,
            },
            壬 => match self {
                天梁 => Some(禄),
                紫微 => Some(权),
                天府 => Some(科),
                武曲 => Some(忌),
                _ => None,
            },
            癸 => match self {
                破军 => Some(禄),
                巨门 => Some(权),
                太阴 => Some(科),
                贪狼 => Some(忌),
                _ => None,
            },
        }
    }

    // 庙、旺、得、利、平、不、陷
    pub fn power(&self, zhi: &DiZhi) -> Option<Power> {
        match zhi {
            子 => match self {
                天机 | 天府 | 太阴 | 天相 | 天梁 | 破军 => Some(庙),
                武曲 | 天同 | 贪狼 | 巨门 | 七杀 => Some(旺),
                文昌 | 文曲 => Some(得),
                紫微 | 廉贞 => Some(平),
                太阳 | 擎羊 | 火星 | 铃星 => Some(陷),
                _ => None,
            },
            丑 => match self {
                紫微 | 武曲 | 天府 | 太阴 | 贪狼 | 天相 | 七杀 | 文昌 | 文曲 | 擎羊 | 陀罗 => {
                    Some(庙)
                }

                天梁 | 破军 => Some(旺),
                火星 | 铃星 => Some(得),
                廉贞 => Some(利),
                // "紫微" | "廉贞" => Some("平".to_string()),
                太阳 | 天同 | 巨门 => Some(不),
                天机 => Some(陷),
                _ => None,
            },
            寅 => match self {
                廉贞 | 天府 | 巨门 | 天相 | 天梁 | 七杀 | 火星 | 铃星 => Some(庙),
                紫微 | 太阳 | 太阴 => Some(旺),
                天机 | 武曲 | 破军 => Some(得),
                天同 => Some(利),
                贪狼 | 文曲 => Some(平),
                // "太阳" | "天同" | "巨门" => Some("不".to_string()),
                文昌 | 陀罗 => Some(陷),
                _ => None,
            },
            卯 => match self {
                太阳 | 巨门 | 天梁 => Some(庙),
                紫微 | 天机 | 七杀 | 文曲 => Some(旺),
                天府 => Some(得),
                武曲 | 贪狼 | 文昌 | 火星 | 铃星 => Some(利),
                天同 | 廉贞 => Some(平),
                // "太阳" | "天同" | "巨门" => Some("不".to_string()),
                太阴 | 天相 | 破军 | 擎羊 => Some(陷),
                _ => None,
            },
            辰 => match self {
                武曲 | 天府 | 贪狼 | 天梁 | 七杀 | 擎羊 | 陀罗 => Some(庙),
                太阳 | 破军 => Some(旺),
                紫微 | 天相 | 文昌 | 文曲 => Some(得),
                天机 | 廉贞 => Some(利),
                天同 => Some(平),
                // "太阳" | "天同" | "巨门" => Some("不".to_string()),
                太阴 | 巨门 | 火星 | 铃星 => Some(陷),
                _ => None,
            },
            巳 => match self {
                天同 | 文昌 | 文曲 => Some(庙),
                紫微 | 太阳 | 巨门 => Some(旺),
                天府 | 天相 | 火星 | 铃星 => Some(得),
                // 天机 | 廉贞 => Some(利),
                天机 | 武曲 | 七杀 | 破军 => Some(平),
                // "太阳" | "天同" | "巨门" => Some("不".to_string()),
                廉贞 | 太阴 | 贪狼 | 天梁 | 陀罗 => Some(陷),
                _ => None,
            },
            午 => match self {
                紫微 | 天机 | 天相 | 天梁 | 破军 | 火星 | 铃星 => Some(庙),
                太阳 | 武曲 | 天府 | 贪狼 | 巨门 | 七杀 => Some(旺),
                // 紫微 | 天相 | 文昌 | 文曲 => Some(得),
                // 天机 | 廉贞 => Some(利),
                廉贞 => Some(平),
                太阴 => Some(不),
                天同 | 文昌 | 文曲 | 擎羊 => Some(陷),
                _ => None,
            },
            未 => match self {
                紫微 | 武曲 | 天府 | 贪狼 | 七杀 | 擎羊 | 陀罗 => Some(庙),
                天梁 | 破军 | 文曲 => Some(旺),
                太阳 | 天相 => Some(得),
                廉贞 | 文昌 | 火星 | 铃星 => Some(利),
                // 天同 => Some(平),
                天同 | 太阴 | 巨门 => Some(不),
                天机 => Some(陷),
                _ => None,
            },
            申 => match self {
                廉贞 | 巨门 | 天相 | 七杀 => Some(庙),
                紫微 | 天同 => Some(旺),
                天机 | 太阳 | 武曲 | 天府 | 破军 | 文昌 | 文曲 => Some(得),
                太阴 => Some(利),
                贪狼 => Some(平),
                // 天同 | 太阴 | 巨门 => Some(不),
                天梁 | 陀罗 | 火星 | 铃星 => Some(陷),
                _ => None,
            },
            酉 => match self {
                巨门 | 文昌 | 文曲 => Some(庙),
                紫微 | 天机 | 天府 | 太阴 | 七杀 => Some(旺),
                天梁 | 火星 | 铃星 => Some(得),
                武曲 | 贪狼 => Some(利),
                太阳 | 廉贞 | 天同 => Some(平),
                // 天同 | 太阴 | 巨门 => Some(不),
                天相 | 破军 | 擎羊 => Some(陷),
                _ => None,
            },
            戌 => match self {
                武曲 | 天府 | 贪狼 | 天梁 | 七杀 | 擎羊 | 陀罗 | 火星 | 铃星 => {
                    Some(庙)
                }
                太阴 | 破军 => Some(旺),
                紫微 | 天相 => Some(得),
                天机 | 廉贞 => Some(利),
                天同 => Some(平),
                太阳 => Some(不),
                巨门 | 文昌 | 文曲 => Some(陷),
                _ => None,
            },
            亥 => match self {
                天同 | 太阴 => Some(庙),
                紫微 | 巨门 | 文曲 => Some(旺),
                天府 | 天相 => Some(得),
                文昌 | 火星 | 铃星 => Some(利),
                天机 | 武曲 | 七杀 | 破军 => Some(平),
                // 天同 | 太阴 | 巨门 => Some(不),
                太阳 | 廉贞 | 贪狼 | 天梁 | 陀罗 => Some(陷),
                _ => None,
            },
        }
    }

    pub fn describe(&self) -> Option<String> {
        match self {
            紫微 => Some("化帝座官禄主".to_string()),
            天机 => Some("化善兄弟主".to_string()),
            太阳 => Some("化贵官禄主".to_string()),
            武曲 => Some("化财财帛主".to_string()),
            天同 => Some("化福福德主".to_string()),
            // 化杀囚在官禄为官禄主在身命为次桃花
            // 化杀囚官禄主桃花
            廉贞 => Some("化杀囚在官禄为官禄主在身命为次桃花".to_string()),
            天府 => Some("化令星财帛田宅主".to_string()),
            太阴 => Some("化富财帛田宅主".to_string()),
            贪狼 => Some("化桃花杀主祸福".to_string()),
            巨门 => Some("化暗主是非".to_string()),
            天相 => Some("化印官禄主".to_string()),
            天梁 => Some("化荫主寿星".to_string()),
            七杀 => Some("将星遇帝为权".to_string()),
            破军 => Some("化耗夫妻子女奴仆".to_string()),
            左辅 => Some("善住雨令星".to_string()),
            右弼 => Some("善住雨令星".to_string()),
            文昌 => Some("司科甲乃文魁之首".to_string()),
            文曲 => Some("主科甲".to_string()),
            天魁 => None,
            天铖 => None,
            禄存 => Some("司爵贵寿星".to_string()),
            擎羊 => Some("化刑".to_string()),
            陀罗 => Some("化忌".to_string()),
            天马 => None,
            火星 => None,
            铃星 => None,
            地空 => None,
            地劫 => None,
            红鸾 => None,
            天喜 => None,
            天姚 => None,
            咸池 => None,
            天刑 => None,
        }
    }

    pub fn wu_xing(&self) -> Vec<WuXing> {
        match self {
            紫微 => vec![土],
            天机 => vec![木],
            太阳 => vec![火],
            武曲 => vec![金],
            天同 => vec![水],
            廉贞 => vec![火],
            天府 => vec![土],
            太阴 => vec![水],
            贪狼 => vec![水, 木],
            巨门 => vec![水],
            天相 => vec![水],
            天梁 => vec![土],
            七杀 => vec![火, 金],
            破军 => vec![水],
            左辅 => vec![土],
            右弼 => vec![土],
            文昌 => vec![金],
            文曲 => vec![水],
            天魁 => vec![],
            天铖 => vec![],
            禄存 => vec![土],
            擎羊 => vec![],
            陀罗 => vec![],
            天马 => vec![],
            火星 => vec![],
            铃星 => vec![],
            地空 => vec![],
            地劫 => vec![],
            红鸾 => vec![],
            天喜 => vec![],
            天姚 => vec![],
            咸池 => vec![],
            天刑 => vec![],
        }
    }
}
