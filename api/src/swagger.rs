use utoipa::OpenApi;

use crate::{
    handlers::__path_ziwei,
    request::{DateRequest, ZiWeiRenReust},
};

use ganzhiwuxing::{DiZhi, GanZhi, TianGan, WuXing};
use lunar_calendar::{LunarCalendar, SolarTerm};
use ziwei::{Power, Star, StarCategory, StarName, Vstar, ZiWei};

use horo_date_time::HoroDateTime;

// swagger
#[derive(OpenApi)]
#[openapi(
    paths(ziwei),
    components(schemas(
        HoroDateTime,

        DiZhi,GanZhi,TianGan,WuXing,
        // 农历
        LunarCalendar,SolarTerm,
        ZiWei,
        DateRequest, ZiWeiRenReust,
        // star
        Star,StarCategory,StarName,Power, Vstar,
        // StarCategory
    ))
)]
pub struct ApiDoc;
