use actix_web::{
    error, get, post,
    web::{self, Redirect},
    Error, Responder,
};
use actix_web_lab::respond::Html;
use entity::power;
use log::info;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

use crate::{request::AddPowerForm, state::AppState};

#[get("/powers")]
pub async fn powers(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    let powers_list = power::Entity::find()
        .all(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;

    let mut ctx = tera::Context::new();
    ctx.insert("powers", &powers_list);
    let s = tmpl
        .render("powers.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;

    Ok(Html(s))
}

#[post("/powers")]
pub async fn add_power(
    app_state: web::Data<AppState>,
    web::Form(form): web::Form<AddPowerForm>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    info!("新增星曜力量：{}", form.name);
    let power = power::Entity::find()
        .filter(power::Column::Name.eq(form.name.clone()))
        .one(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;

    if power.is_none() {
        power::ActiveModel {
            name: ActiveValue::Set(form.name),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;
    }
    Ok(Redirect::to("/powers").see_other())
}

#[post("/powers/{id}")]
pub async fn delete_power(
    id: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    let id = id.into_inner();
    info!("删除星曜力量：id={}", id);
    let power = power::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;

    if power.is_some() {
        power::Entity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;
    }
    Ok(Redirect::to("/powers").see_other())
}
