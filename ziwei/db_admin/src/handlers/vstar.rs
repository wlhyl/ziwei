use actix_web::{
    error, get, post,
    web::{self, Redirect},
    Error, Responder,
};
use actix_web_lab::respond::Html;
use entity::vstar;
use log::info;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

use crate::{request::AddVstarForm, state::AppState};

#[get("/vstars")]
pub async fn vstars(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    let vstars_list = vstar::Entity::find()
        .all(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;

    let mut ctx = tera::Context::new();
    ctx.insert("vstars", &vstars_list);
    let s = tmpl
        .render("vstars.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;

    Ok(Html(s))
}

#[post("/vstars")]
pub async fn add_vstar(
    app_state: web::Data<AppState>,
    web::Form(form): web::Form<AddVstarForm>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    info!("新增化曜：{}", form.name);
    let vstar = vstar::Entity::find()
        .filter(vstar::Column::Name.eq(form.name.clone()))
        .one(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;

    if vstar.is_none() {
        vstar::ActiveModel {
            name: ActiveValue::Set(form.name),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;
    }
    Ok(Redirect::to("/vstars").see_other())
}

#[post("/vstars/{id}")]
pub async fn delete_vstar(
    id: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    let id = id.into_inner();
    info!("删除宫位{}", id);
    let vstar = vstar::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;

    if vstar.is_some() {
        vstar::Entity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;
    }
    Ok(Redirect::to("/vstars").see_other())
}
