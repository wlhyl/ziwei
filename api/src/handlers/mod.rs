use actix_web::{post, web, HttpResponse, Responder};

use crate::{handlers::request::ZiWeiRenReust, state::AppState, ziwei::ZiWeiPan};

pub mod healthz;

pub mod request;

/// 紫微斗数排盘
///
/// method: post
///
/// api: /daliuren
#[utoipa::path(
    context_path="/api",
    request_body=ZiWeiRenReust,
    responses(
        (status = 200, description = "OK", body = ZiWeiPan),
    ),
    security(
        ("api_key" = [])
    ),
)]
#[post("/ziwei")]
pub async fn ziwei(
    app_state: web::Data<AppState>,
    r: actix_web_validator::Json<ZiWeiRenReust>,
) -> impl Responder {
    let r = r.into_inner();

    let pan = ZiWeiPan::new(
        r.year,
        r.month,
        r.day,
        r.hour,
        r.minute,
        r.second,
        r.masculine,
        r.process_year,
        r.process_month,
        r.process_day,
        r.process_hour,
        r.process_minute,
        r.process_second,
        &app_state.ephe_path,
    );
    match pan {
        Ok(pan) => HttpResponse::Ok().json(pan),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}
