use actix_web::{
    error, get,
    http::{header::LOCATION, StatusCode},
    post, web, Error, HttpResponse, Responder,
};
use actix_web_lab::respond::Html;
use entity::gan;
use log::info;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

use crate::{request::AddGanForm, state::AppState};

/// 获取所有天干
#[get("/gan")]
pub async fn tan_gan(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    let mut ctx = tera::Context::new();
    ctx.insert("error_message", "");

    match gan::Entity::find().all(db).await {
        Ok(v) => ctx.insert("gan", &v),
        Err(e) => {
            let v: Vec<gan::Model> = vec![];
            ctx.insert("gan", &v);
            ctx.insert("error_message", &e.to_string());
        }
    }

    let s = tmpl
        .render("gan.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;

    Ok(Html(s))
}

/// 新增天干
#[post("/gan")]
pub async fn add_tian_gan(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    web::Form(form): web::Form<AddGanForm>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;

    let mut ctx = tera::Context::new();
    ctx.insert("error_message", "");

    info!("新增天干：{}", form.name);
    // 检查天干是否已经存在
    let g = match gan::Entity::find()
        .filter(gan::Column::Name.eq(form.name.clone()))
        .one(db)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            // let v: Vec<gan::Model> = vec![];
            // ctx.insert("gan", &v);
            ctx.insert("error_message", &e.to_string());
            let s = tmpl
                .render("gan.html", &ctx)
                .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
            return Ok(HttpResponse::Ok().body(s));
        }
    };

    if g.is_none() {
        let g = gan::ActiveModel {
            name: ActiveValue::Set(form.name),
            ..Default::default()
        };
        if let Err(e) = g.insert(db).await {
            // let v: Vec<gan::Model> = vec![];
            // ctx.insert("gan", &v);
            ctx.insert("error_message", &e.to_string());
            let s = tmpl
                .render("gan.html", &ctx)
                .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
            return Ok(HttpResponse::Ok().body(s));
        }
    }

    let res = HttpResponse::PermanentRedirect()
        .insert_header((LOCATION, "/gan"))
        .status(StatusCode::SEE_OTHER) // 不加这行，仍以post重定向
        .finish();
    Ok(res)
}

/// 删除天干
#[post("/gan/{id}")]
pub async fn delete_gan(
    id: web::Path<i32>,
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    let id = id.into_inner();
    info!("删除天干：id={}", id);
    let mut ctx = tera::Context::new();
    ctx.insert("error_message", "");

    let g = match gan::Entity::find_by_id(id).one(db).await {
        Ok(v) => v,
        Err(e) => {
            ctx.insert("error_message", &e.to_string());
            let s = tmpl
                .render("gan.html", &ctx)
                .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
            return Ok(HttpResponse::Ok().body(s));
        }
    };

    if g.is_some() {
        if let Err(e) = gan::Entity::delete_by_id(id).exec(db).await {
            ctx.insert("error_message", &e.to_string());
            let s = tmpl
                .render("gan.html", &ctx)
                .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
            return Ok(HttpResponse::Ok().body(s));
        }
    }
    let res = HttpResponse::PermanentRedirect()
        .insert_header((LOCATION, "/gan"))
        .status(StatusCode::SEE_OTHER) // 不加这行，仍以post重定向
        .finish();
    Ok(res)
}
