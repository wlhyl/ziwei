use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct ZiWeiRenReust {
    /// 年，最小值1900
    #[validate(range(min = 1900))]
    pub year: i32,
    /// 月
    #[validate(range(min = 1, max = 12))]
    pub month: u8,
    #[validate(range(min = 1, max = 31))]
    /// 日
    pub day: u8,
    /// 时
    #[validate(range(min = 0, max = 23))]
    pub hour: u8,
    /// 分
    #[validate(range(min = 0, max = 59))]
    pub minute: u8,
    /// 秒
    #[validate(range(min = 0, max = 59))]
    pub second: u8,

    /// 性别，男：true，女：false
    pub masculine: bool,

    /// 推运年，最小值1900
    #[validate(range(min = 1900))]
    pub process_year: i32,
    /// 推运月
    #[validate(range(min = 1, max = 12))]
    pub process_month: u8,
    #[validate(range(min = 1, max = 31))]
    /// 推运日
    pub process_day: u8,
    /// 推运时
    #[validate(range(min = 0, max = 23))]
    pub process_hour: u8,
    /// 推运分
    #[validate(range(min = 0, max = 59))]
    pub process_minute: u8,
    /// 推运秒
    #[validate(range(min = 0, max = 59))]
    pub process_second: u8,
}
