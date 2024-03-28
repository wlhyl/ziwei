use actix_web::{get, web::{self, Redirect}, Responder, Error, error, post};
use actix_web_lab::respond::Html;
use entity::house;
use log::info;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveValue, ActiveModelTrait};

use crate::{state::AppState, request::AddHouseForm};

#[get("/houses")]
pub async fn houses(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    let houses_list = house::Entity::find()
        .all(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;

    // let ctx = tera::Context::from_serialize(houses_list).unwrap();
    // .map_err(|error| error::ErrorInternalServerError(error))?;
    let mut ctx = tera::Context::new();
    ctx.insert("houses", &houses_list);
    let s = tmpl
        .render("houses.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(Html(s))
}

#[post("/houses")]
pub async fn add_house(
    app_state: web::Data<AppState>,
    web::Form(form): web::Form<AddHouseForm>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    info!("新增宫位：{}", form.name);
    let house = house::Entity::find()
        .filter(house::Column::Name.eq(form.name.clone()))
        .one(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;

    if house.is_none() {
        house::ActiveModel {
            name: ActiveValue::Set(form.name),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;
    }
    Ok(Redirect::to("/houses").see_other())
}

#[post("/houses/{id}")]
pub async fn delete_house(
    id: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    let id = id.into_inner();
    info!("删除宫位{}", id);
    let house = house::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;

    if house.is_some() {
        house::Entity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;
    }
    Ok(Redirect::to("/houses").see_other())
}