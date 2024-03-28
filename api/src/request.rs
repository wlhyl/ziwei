use serde::Deserialize;
use validator::Validate;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[derive(Deserialize, Validate)]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct DateRequest {
    /// 年，最小值1900
    #[validate(range(min = 1900, message = "年最小1900"))]
    pub year: i32,
    /// 月
    #[validate(range(min = 1, max = 12, message = "1<=月份<=12"))]
    pub month: u8,
    /// 日
    #[validate(range(min = 1, max = 31, message = "1<=日期<=31"))]
    pub day: u8,
    /// 时
    #[validate(range(min = 0, max = 23, message = "0<=时<=23"))]
    pub hour: u8,
    /// 分
    #[validate(range(min = 0, max = 59, message = "0<=分<=59"))]
    pub minute: u8,
    /// 秒
    #[validate(range(min = 0, max = 59, message = "0<=秒<=59"))]
    pub second: u8,
}

#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Deserialize, Validate)]
pub struct ZiWeiRenReust {
    /// 出生时间
    #[validate]
    pub native_date: DateRequest,

    /// 推运时间
    #[validate]
    pub process_date: DateRequest,

    /// 性别，男：true，女：false
    pub masculine: bool,
}
