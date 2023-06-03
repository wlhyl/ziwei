use utoipa::OpenApi;

use crate::{
    handlers::{__path_ziwei, request::ZiWeiRenReust},
    ziwei::{
        star::{Star, StarCategory},
        LunarCalendar, SiZhu, SolarTerm, ZiWeiPan,
    },
};
// swagger
#[derive(OpenApi)]
#[openapi(
    paths(ziwei),
    components(schemas(
        ZiWeiPan,
        LunarCalendar,
        SolarTerm,
        SiZhu,
        ZiWeiRenReust,
        Star,
        StarCategory
    ))
)]
pub struct ApiDoc;
