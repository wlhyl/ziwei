use actix_web::{
    error, get, post,
    web::{self, Redirect},
    Error, Responder,
};
use actix_web_lab::respond::Html;
use entity::category;
use log::{info, warn};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use validator::Validate;

use crate::{
    request::{AddCategoryForm, EditingCategory, UpdateCategoryForm},
    state::AppState,
};

#[get("/categories")]
pub async fn categories(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    let categories_list = category::Entity::find()
        .all(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;

    let mut ctx = tera::Context::new();
    ctx.insert("categories", &categories_list);

    let s = tmpl
        .render("category/categories.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;

    Ok(Html(s))
}

#[post("/categories")]
pub async fn add_category(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    web::Form(form): web::Form<AddCategoryForm>,
) -> Result<impl Responder, Error> {
    info!("新增星曜分类：{}", form.name);
    let db = &app_state.db;

    let mut ctx = tera::Context::new();
    ctx.insert("name_error", "");
    ctx.insert("describe_error", "");

    // 此处应该重定向到/categories?name=xx&describe=xx&name_error=true&describe_error=true
    if let Err(error) = form.validate() {
        let error_map = error.field_errors();

        if error_map.get("name").is_some() {
            ctx.insert("name_error", "name不能为空字符串，或控制字符");
        }

        if error_map.get("describe").is_some() {
            ctx.insert("describe_error", "describe不能是控制字符");
        }
    } else {
        let category = category::Entity::find()
            .filter(category::Column::Name.eq(form.name.clone()))
            .one(db)
            .await
            .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;

        if category.is_none() {
            category::ActiveModel {
                name: ActiveValue::Set(form.name),
                describe: if let Some(describe) = form.describe {
                    if describe.is_empty() {
                        ActiveValue::set(None)
                    } else {
                        ActiveValue::Set(Some(describe))
                    }
                } else {
                    ActiveValue::set(None)
                },
                ..Default::default()
            }
            .insert(db)
            .await
            .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;
        }
    }

    let categories_list = category::Entity::find()
        .all(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;

    ctx.insert("categories", &categories_list);

    let s = tmpl
        .render("category/add_category.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
    Ok(Html(s))
}

#[post("/categories/delete/{id}")]
pub async fn delete_category(
    id: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    let id = id.into_inner();
    info!("删除星曜分类：id={}", id);
    let category = category::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;

    if category.is_some() {
        category::Entity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;
    }
    Ok(Redirect::to("/categories").see_other())
}

/// 生成星曜分类编辑页面
#[get("/categories/edit/{id}")]
pub async fn edit_category(
    id: web::Path<i32>,
    updated_category: web::Query<EditingCategory>,
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    info!("生成编辑星曜分类页面");
    let db = &app_state.db;
    let id = id.into_inner();
    let updated_category = updated_category.into_inner();

    let categories_list = category::Entity::find()
        .all(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;

    let mut ctx = tera::Context::new();
    ctx.insert("categories", &categories_list);
    ctx.insert("name_error", "");
    ctx.insert("describe_error", "");
    ctx.insert("error_message", "");

    let mut category = if let Some(category) = categories_list.iter().find(|c| c.id == id) {
        category.clone()
    } else {
        ctx.insert("error_message", "找不到分类");
        category::Model {
            id,
            name: "".to_string(),
            describe: None,
        }
    };

    if let Some(name) = updated_category.name {
        category.name = name;
    }

    if updated_category.describe.is_some() {
        category.describe = updated_category.describe;
    }

    ctx.insert("editing_category", &category);

    if updated_category.name_error.is_some() {
        ctx.insert("name_error", "name不能为空字符串，或控制字符");
    }

    if updated_category.describe_error.is_some() {
        ctx.insert("describe_error", "describe不能是控制字符");
    }

    let s = tmpl
        .render("category/edit_category.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
    Ok(Html(s))
}

// 修改category
#[post("/categories/{id}")]
pub async fn update_category(
    id: web::Path<i32>,
    web::Form(form): web::Form<UpdateCategoryForm>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    let id = id.into_inner();
    info!("修改星曜分类：id={}", id);

    if let Err(error) = form.validate() {
        let mut url = format!(
            "/categories/edit/{}?name={}&describe={}",
            id, form.name, form.describe
        );

        let error_map = error.field_errors();

        if error_map.get("name").is_some() {
            url = format!("{}&name_error=true", url);
        }

        if error_map.get("describe").is_some() {
            format!("{}&describe_error=true", url);
        }

        return Ok(Redirect::to(url).see_other());
    }

    let category = category::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;

    if let Some(category) = category {
        info!("更新星曜分类：id={}", id);
        let mut category: category::ActiveModel = category.into();
        category.name = ActiveValue::Set(form.name);
        if !form.describe.is_empty() {
            category.describe = ActiveValue::Set(Some(form.describe));
        }

        category
            .update(db)
            .await
            .map_err(|error| error::ErrorInternalServerError(error.to_string()))?;
    } else {
        warn!("找不到星曜id={}，没有更新", id);
    }

    Ok(Redirect::to("/categories").see_other())

    // let res = HttpResponse::PermanentRedirect()
    //     .insert_header((LOCATION, "/categories"))
    //     .finish();
    // Ok(res)
}
