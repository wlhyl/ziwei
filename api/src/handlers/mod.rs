use ::ziwei::ZiWei;
use actix_web::{post, web, HttpResponse, Responder};
use horo_date_time::horo_date_time;

use crate::error::Error;
use crate::{request::ZiWeiRenReust, state::AppState};

pub mod healthz;

/// 紫微斗数排盘
#[cfg_attr(feature = "swagger", 
utoipa::path(
    context_path="/api",
    request_body=ZiWeiRenReust,
    responses(
        (status = 200, description = "返回紫微盘", body = ZiWei),
    ),
)
)]
#[post("/ziwei")]
pub async fn ziwei(
    app_state: web::Data<AppState>,
    r: actix_web_validator::Json<ZiWeiRenReust>,
) -> Result<impl Responder, Error> {
    let r = r.into_inner();

    let native_date = horo_date_time(
        r.native_date.year,
        r.native_date.month,
        r.native_date.day,
        r.native_date.hour,
        r.native_date.minute,
        r.native_date.second,
        8.0,
        false,
    )?;
    let process_date = horo_date_time(
        r.process_date.year,
        r.process_date.month,
        r.process_date.day,
        r.process_date.hour,
        r.process_date.minute,
        r.process_date.second,
        8.0,
        false,
    )?;

    let pan = ZiWei::new(native_date, process_date, r.masculine, &app_state.ephe_path)?;

    Ok(HttpResponse::Ok().json(pan))
}
