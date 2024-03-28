use actix_web::{delete, error, get, post, put, web, Error, HttpResponse, Responder};
use actix_web_lab::respond::Html;
use entity::{category, gan, gan_star_vstar, house, power, star, star_house_power, vstar};
use log::{info, warn};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
    QuerySelect,
};

use crate::{
    request::{
        EditStarForm, EditStarPowerQuery, EditStarVstarQuery, GanStarVstarRequest, StarFormRequest,
        StarHousePowerRequest, StarsQuery,
    },
    responser::{GanStarVstarResponser, StarHousePowerResponser},
    state::AppState,
};

#[get("/stars")]
pub async fn stars(
    query: web::Query<StarsQuery>,
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    let query = query.into_inner();

    let mut ctx = tera::Context::new();
    ctx.insert("error_message", "");

    let query = if let Some(query) = query.list {
        query
    } else {
        "".to_string()
    };

    if query == "powers" {
        // 查询星曜，用于新增power
        match star::Entity::find().all(db).await {
            Ok(v) => ctx.insert("stars", &v),
            Err(e) => {
                ctx.insert("error_message", &e.to_string());
                let s = tmpl.render("star/stars_powers.html", &ctx).map_err(|e| {
                    error::ErrorInternalServerError(format!("Template error:{}", e))
                })?;
                return Ok(Html(s));
            }
        }

        // 查询宫位，用于新增power
        match house::Entity::find().all(db).await {
            Ok(v) => ctx.insert("houses", &v),
            Err(e) => {
                ctx.insert("error_message", &e.to_string());
                let s = tmpl.render("star/stars_powers.html", &ctx).map_err(|e| {
                    error::ErrorInternalServerError(format!("Template error:{}", e))
                })?;
                return Ok(Html(s));
            }
        }

        // 查询power，用于新增power
        match power::Entity::find().all(db).await {
            Ok(v) => ctx.insert("powers", &v),
            Err(e) => {
                ctx.insert("error_message", &e.to_string());
                let s = tmpl.render("star/stars_powers.html", &ctx).map_err(|e| {
                    error::ErrorInternalServerError(format!("Template error:{}", e))
                })?;
                return Ok(Html(s));
            }
        }
        // star::Entity::find()
        //     .join(JoinType::LeftJoin, star::Relation::StarHousePower.def())
        //     .join(JoinType::LeftJoin, star_house_power::Relation::House.def())
        //     .join(JoinType::LeftJoin, star_house_power::Relation::Power.def())
        //     .select_only()
        //     .column_as(star::Column::Id, "star_id")
        //     .column_as(star::Column::Name, "star")
        //     .column_as(house::Column::Name, "house")
        //     .column_as(power::Column::Name, "power")
        //     .build(DbBackend::Sqlite)
        //     .to_string();
        match star_house_power::Entity::find()
            .left_join(star::Entity)
            .left_join(house::Entity)
            .left_join(power::Entity)
            .select_only()
            .column_as(star::Column::Id, "star_id")
            .column_as(house::Column::Id, "house_id")
            .column_as(star::Column::Name, "star")
            .column_as(house::Column::Name, "house")
            .column_as(power::Column::Name, "power")
            .into_model::<StarHousePowerResponser>()
            .all(db)
            .await
        {
            Ok(v) => {
                ctx.insert("star_powers", &v);
            }
            Err(e) => {
                ctx.insert("error_message", &e.to_string());
            }
        };

        let s = tmpl
            .render("star/stars_powers.html", &ctx)
            .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
        return Ok(Html(s));
    } else if query == "vstars" {
        // 查询天干，用于新增vstar
        match gan::Entity::find().all(db).await {
            Ok(v) => ctx.insert("gan", &v),
            Err(e) => {
                ctx.insert("error_message", &e.to_string());
                let s = tmpl.render("star/star_vstar.html", &ctx).map_err(|e| {
                    error::ErrorInternalServerError(format!("Template error:{}", e))
                })?;
                return Ok(Html(s));
            }
        }

        // 查询星曜，用于新增vstar
        match star::Entity::find().all(db).await {
            Ok(v) => ctx.insert("stars", &v),
            Err(e) => {
                ctx.insert("error_message", &e.to_string());
                let s = tmpl.render("star/star_vstar.html", &ctx).map_err(|e| {
                    error::ErrorInternalServerError(format!("Template error:{}", e))
                })?;
                return Ok(Html(s));
            }
        }

        // 查询vstar，用于新增vstar
        match vstar::Entity::find().all(db).await {
            Ok(v) => ctx.insert("vstars", &v),
            Err(e) => {
                ctx.insert("error_message", &e.to_string());
                let s = tmpl.render("star/star_vstar.html", &ctx).map_err(|e| {
                    error::ErrorInternalServerError(format!("Template error:{}", e))
                })?;
                return Ok(Html(s));
            }
        }

        match gan_star_vstar::Entity::find()
            .left_join(star::Entity)
            .left_join(gan::Entity)
            .left_join(vstar::Entity)
            .select_only()
            .column_as(gan::Column::Id, "gan_id")
            .column_as(star::Column::Id, "star_id")
            .column_as(gan::Column::Name, "gan")
            .column_as(star::Column::Name, "star")
            .column_as(vstar::Column::Name, "vstar")
            .into_model::<GanStarVstarResponser>()
            .all(db)
            .await
        {
            Ok(v) => {
                ctx.insert("star_vstars", &v);
            }
            Err(e) => {
                ctx.insert("error_message", &e.to_string());
            }
        };

        let s = tmpl
            .render("star/star_vstar.html", &ctx)
            .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
        return Ok(Html(s));
    } else {
        match star::Entity::find()
            .find_also_related(category::Entity)
            .all(db)
            .await
        {
            Ok(v) => {
                ctx.insert("stars", &v);
            }
            Err(e) => {
                // let v: Vec<(star::Model, Option<category::Model>)> = vec![];
                // ctx.insert("stars", &v);
                ctx.insert("error_message", &e.to_string());
            }
        }

        let s = tmpl
            .render("star/stars.html", &ctx)
            .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
        return Ok(Html(s));
    };
}

/// 生成编辑star页面
#[get("/stars/edit")]
pub async fn edit_star(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    web::Query(form): web::Query<EditStarForm>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    let tmpl_path = "star/edit_star.html";
    let mut ctx = tera::Context::new();
    ctx.insert("id", &0);
    ctx.insert("name", "");
    ctx.insert("category", &0);
    ctx.insert("wu_xing", "");
    ctx.insert("lord", "");
    ctx.insert("describe", "");
    ctx.insert("error_message", "");

    match category::Entity::find().all(db).await {
        Ok(v) => {
            ctx.insert("categories", &v);
        }
        Err(e) => {
            let v: Vec<category::Model> = vec![];
            ctx.insert("categories", &v);
            ctx.insert("error_message", &e.to_string());

            let s = tmpl
                .render(tmpl_path, &ctx)
                .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
            return Ok(Html(s));
        }
    }

    let id = if let Some(id) = form.id {
        id
    } else {
        info!("生成新增星曜页面");

        let s = tmpl
            .render(tmpl_path, &ctx)
            .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
        return Ok(Html(s));
    };

    info!("编辑星曜：{}", id);
    let star = match star::Entity::find_by_id(id).one(db).await {
        Ok(v) => v,
        Err(e) => {
            ctx.insert("error_message", &e.to_string());

            let s = tmpl
                .render(tmpl_path, &ctx)
                .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
            return Ok(Html(s));
        }
    };

    let star = if let Some(star) = star {
        star
    } else {
        ctx.insert("error_message", &format!("没有id={}的星曜", id));
        let s = tmpl
            .render(tmpl_path, &ctx)
            .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
        return Ok(Html(s));
    };

    ctx.insert("id", &id);
    ctx.insert("name", &star.name);
    ctx.insert("category", &star.category_id);
    ctx.insert("wu_xing", &star.wu_xing);
    ctx.insert("lord", &star.lord);
    ctx.insert("describe", &star.describe);

    let s = tmpl
        .render(tmpl_path, &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
    Ok(Html(s))
}

/// 新增star
#[post("/api/stars")]
pub async fn add_star(
    app_state: web::Data<AppState>,
    actix_web_validator::Json(r): actix_web_validator::Json<StarFormRequest>,
) -> impl Responder {
    let db = &app_state.db;

    // category_id 是否存在
    info!("检查category id{}是否存在", r.category_id);
    let category = match category::Entity::find_by_id(r.category_id).one(db).await {
        Ok(v) => v,
        Err(e) => return HttpResponse::InternalServerError().body(format!("数据库错误:{}", e)),
    };

    if category.is_none() {
        warn!("新增star, category id: {}不存在", r.category_id);
        return HttpResponse::BadRequest().body(format!("category id:{}不存在在", r.category_id));
    }

    // 确保name不重复
    match star::Entity::find()
        .filter(star::Column::Name.eq(r.name.clone()))
        .one(db)
        .await
    {
        Ok(v) => {
            if v.is_some() {
                return HttpResponse::BadRequest().body(format!("name:{}已经存在在", r.name));
            }
        }
        Err(e) => return HttpResponse::InternalServerError().body(format!("数据库错误:{}", e)),
    };

    info!("新增星曜:{}", r.name);

    let star_entity = star::ActiveModel {
        name: ActiveValue::Set(r.name),
        category_id: ActiveValue::Set(r.category_id),
        wu_xing: ActiveValue::Set(r.wu_xing),
        lord: if r.lord.is_some() {
            ActiveValue::Set(r.lord)
        } else {
            ActiveValue::NotSet
        },
        describe: if r.describe.is_some() {
            ActiveValue::Set(r.describe)
        } else {
            ActiveValue::NotSet
        },
        ..Default::default()
    };

    if let Err(e) = star_entity.insert(db).await {
        return HttpResponse::InternalServerError().body(format!("数据库错误：{}", e));
    }

    HttpResponse::Ok().finish()
}

/// 修改star
#[put("/api/stars/{id}")]
pub async fn update_star(
    id: web::Path<i32>,
    app_state: web::Data<AppState>,
    actix_web_validator::Json(r): actix_web_validator::Json<StarFormRequest>,
) -> impl Responder {
    let db = &app_state.db;
    let id = id.into_inner();

    // category_id 是否存在
    info!("检查category id{}是否存在", r.category_id);
    let category = match category::Entity::find_by_id(r.category_id).one(db).await {
        Ok(v) => v,
        Err(e) => return HttpResponse::InternalServerError().body(format!("数据库错误:{}", e)),
    };

    if category.is_none() {
        warn!("修改star, category id: {}不存在", r.category_id);
        return HttpResponse::BadRequest().body(format!("category id:{}不存在在", r.category_id));
    }

    // 确保记录存在
    let star = match star::Entity::find_by_id(id).one(db).await {
        Ok(v) => {
            if let Some(v) = v {
                v
            } else {
                return HttpResponse::BadRequest().body(format!("star id:{}不存在在", id));
            }
        }

        Err(e) => return HttpResponse::InternalServerError().body(format!("数据库错误:{}", e)),
    };

    info!("修改星曜id:{}", id);

    let mut star = star.into_active_model();

    star.category_id = ActiveValue::Set(r.category_id);
    star.wu_xing = ActiveValue::Set(r.wu_xing);
    star.lord = ActiveValue::Set(r.lord);
    star.describe = ActiveValue::Set(r.describe);

    if let Err(e) = star.update(db).await {
        return HttpResponse::InternalServerError().body(format!("数据库错误：{}", e));
    }
    HttpResponse::Ok().finish()
}

/// 删除star
#[delete("/api/stars/{star_id}")]
pub async fn delete_star(
    app_state: web::Data<AppState>,
    star_id: web::Path<i32>,
) -> impl Responder {
    let db = &app_state.db;
    let star_id = star_id.into_inner();

    // power =f f( (star, power) )，因此只需查询 star, power
    // 不存在，返回错误
    let star = match star::Entity::find_by_id(star_id).one(db).await {
        Ok(v) => v,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    let star = if let Some(star) = star {
        star
    } else {
        return HttpResponse::BadRequest().body("记录不存在");
    };

    if let Err(e) = star::Entity::delete(star.into_active_model())
        .exec(db)
        .await
    {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Ok().finish()
}

/// 新增/修改star,power
/// 返回json
#[post("/api/stars/power")]
pub async fn add_star_power(
    app_state: web::Data<AppState>,
    actix_web_validator::Json(r): actix_web_validator::Json<StarHousePowerRequest>,
) -> impl Responder {
    let db = &app_state.db;

    match star::Entity::find_by_id(r.star_id).one(db).await {
        Ok(v) => {
            if v.is_none() {
                return HttpResponse::BadRequest().body(format!("star_id:{}不存在", r.star_id));
            }
        }
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    }

    match house::Entity::find_by_id(r.house_id).one(db).await {
        Ok(v) => {
            if v.is_none() {
                return HttpResponse::BadRequest().body(format!("house_id:{}不存在", r.house_id));
            }
        }
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    }

    match power::Entity::find_by_id(r.power_id).one(db).await {
        Ok(v) => {
            if v.is_none() {
                return HttpResponse::BadRequest().body(format!("power_id:{}不存在", r.power_id));
            }
        }
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    }

    // power =f f( (star, power) )，因此只需查询 star, power
    // 已经存在，则不能再添加
    let power = match star_house_power::Entity::find()
        .filter(star_house_power::Column::StarId.eq(r.star_id))
        .filter(star_house_power::Column::HouseId.eq(r.house_id))
        .one(db)
        .await
    {
        Ok(v) => v,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    if power.is_some() {
        return HttpResponse::BadRequest().body("记录已经存在");
    }

    if let Err(e) = (star_house_power::ActiveModel {
        star_id: ActiveValue::Set(r.star_id),
        house_id: ActiveValue::Set(r.house_id),
        power_id: ActiveValue::Set(r.power_id),
        ..Default::default()
    }
    .insert(db)
    .await)
    {
        return HttpResponse::InternalServerError().json(e.to_string());
    }

    HttpResponse::Ok().finish()
}

/// 生成星曜力量编辑页面
#[get("/stars/powers/edit")]
pub async fn edit_star_power(
    web::Query(r): web::Query<EditStarPowerQuery>,
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    info!("生成编辑星曜力量页面");
    let db = &app_state.db;
    let mut ctx = tera::Context::new();
    ctx.insert("edit_star_id", &r.star_id);
    ctx.insert("edit_house_id", &r.house_id);
    ctx.insert("error_message", "");

    // 查询星曜，用于修改power
    match star::Entity::find().all(db).await {
        Ok(v) => ctx.insert("stars", &v),
        Err(e) => {
            ctx.insert("error_message", &e.to_string());
            let s = tmpl
                .render("star/edit_star_power.html", &ctx)
                .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
            return Ok(Html(s));
        }
    }

    // 查询宫位，用于修改power
    match house::Entity::find().all(db).await {
        Ok(v) => ctx.insert("houses", &v),
        Err(e) => {
            ctx.insert("error_message", &e.to_string());
            let s = tmpl
                .render("star/edit_star_power.html", &ctx)
                .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
            return Ok(Html(s));
        }
    }

    // 查询power，用于修改power
    match power::Entity::find().all(db).await {
        Ok(v) => ctx.insert("powers", &v),
        Err(e) => {
            ctx.insert("error_message", &e.to_string());
            let s = tmpl
                .render("star/edit_star_power.html", &ctx)
                .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
            return Ok(Html(s));
        }
    }

    match star_house_power::Entity::find()
        .left_join(star::Entity)
        .left_join(house::Entity)
        .left_join(power::Entity)
        .select_only()
        .column_as(star::Column::Id, "star_id")
        .column_as(house::Column::Id, "house_id")
        .column_as(star::Column::Name, "star")
        .column_as(house::Column::Name, "house")
        .column_as(power::Column::Name, "power")
        .into_model::<StarHousePowerResponser>()
        .all(db)
        .await
    {
        Ok(v) => {
            ctx.insert("star_powers", &v);
        }
        Err(e) => {
            ctx.insert("error_message", &e.to_string());
        }
    };

    let s = tmpl
        .render("star/edit_star_power.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
    return Ok(Html(s));
}

/// 更新star,power
#[put("/api/stars/power")]
pub async fn update_star_power(
    app_state: web::Data<AppState>,
    actix_web_validator::Json(r): actix_web_validator::Json<StarHousePowerRequest>,
) -> impl Responder {
    let db = &app_state.db;

    match power::Entity::find_by_id(r.power_id).one(db).await {
        Ok(v) => {
            if v.is_none() {
                return HttpResponse::BadRequest().body(format!("power_id:{}不存在", r.power_id));
            }
        }
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    }

    // power =f f( (star, power) )，因此只需查询 star, power
    // 不存在，返回错误
    let power = match star_house_power::Entity::find()
        .filter(star_house_power::Column::StarId.eq(r.star_id))
        .filter(star_house_power::Column::HouseId.eq(r.house_id))
        .one(db)
        .await
    {
        Ok(v) => v,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    let power = if let Some(power) = power {
        power
    } else {
        return HttpResponse::BadRequest().body("记录不存在");
    };

    let mut power: star_house_power::ActiveModel = power.into();

    power.power_id = ActiveValue::Set(r.power_id);
    if let Err(e) = power.update(db).await {
        return HttpResponse::InternalServerError().json(e.to_string());
    }

    HttpResponse::Ok().finish()
}

/// 删除star,power
#[delete("/api/stars/power/{star_id},{house_id}")]
pub async fn delete_star_power(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> impl Responder {
    let db = &app_state.db;
    let star_id = params.0;
    let house_id = params.1;

    // power =f f( (star, power) )，因此只需查询 star, power
    // 不存在，返回错误
    let power = match star_house_power::Entity::find()
        .filter(star_house_power::Column::StarId.eq(star_id))
        .filter(star_house_power::Column::HouseId.eq(house_id))
        .one(db)
        .await
    {
        Ok(v) => v,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    let power = if let Some(power) = power {
        power
    } else {
        return HttpResponse::BadRequest().body("记录不存在");
    };

    if let Err(e) = star_house_power::Entity::delete(power.into_active_model())
        .exec(db)
        .await
    {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Ok().finish()
}

/// 新增star,vstar
#[post("/api/stars/vstar")]
pub async fn add_star_vstar(
    app_state: web::Data<AppState>,
    actix_web_validator::Json(r): actix_web_validator::Json<GanStarVstarRequest>,
) -> impl Responder {
    let db = &app_state.db;

    match gan_star_vstar::Entity::find()
        .filter(gan_star_vstar::Column::GanId.eq(r.gan_id))
        .filter(gan_star_vstar::Column::StarId.eq(r.star_id))
        .one(db)
        .await
    {
        Ok(v) => {
            if v.is_some() {
                return HttpResponse::BadRequest().body("记录已经存在");
            }
        }
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    }

    match gan::Entity::find_by_id(r.gan_id).one(db).await {
        Ok(v) => {
            if v.is_none() {
                return HttpResponse::BadRequest().body(format!("gan_id:{}不存在", r.gan_id));
            }
        }
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    }

    match star::Entity::find_by_id(r.star_id).one(db).await {
        Ok(v) => {
            if v.is_none() {
                return HttpResponse::BadRequest().body(format!("star_id:{}不存在", r.star_id));
            }
        }
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    }

    match vstar::Entity::find_by_id(r.vstar_id).one(db).await {
        Ok(v) => {
            if v.is_none() {
                return HttpResponse::BadRequest().body(format!("vstar_id:{}不存在", r.vstar_id));
            }
        }
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    }

    let vstar = gan_star_vstar::ActiveModel {
        gan_id: ActiveValue::Set(r.gan_id),
        star_id: ActiveValue::Set(r.star_id),
        v_star_id: ActiveValue::Set(r.vstar_id),
        ..Default::default()
    };

    if let Err(e) = vstar.insert(db).await {
        return HttpResponse::InternalServerError().json(e.to_string());
    }

    HttpResponse::Ok().finish()
}

/// 更新star,vstar
#[put("/api/stars/vstar")]
pub async fn update_star_vstar(
    app_state: web::Data<AppState>,
    actix_web_validator::Json(r): actix_web_validator::Json<GanStarVstarRequest>,
) -> impl Responder {
    let db = &app_state.db;

    let vstar = match gan_star_vstar::Entity::find()
        .filter(gan_star_vstar::Column::GanId.eq(r.gan_id))
        .filter(gan_star_vstar::Column::StarId.eq(r.star_id))
        .one(db)
        .await
    {
        Ok(v) => v,

        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    let vstar = if let Some(vstar) = vstar {
        vstar
    } else {
        return HttpResponse::BadRequest().body("记录不存在");
    };

    match vstar::Entity::find_by_id(r.vstar_id).one(db).await {
        Ok(v) => {
            if v.is_none() {
                return HttpResponse::BadRequest().body(format!("vstar_id:{}不存在", r.vstar_id));
            }
        }
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    }

    let mut vstar = vstar.into_active_model();
    vstar.v_star_id = ActiveValue::Set(r.vstar_id);

    if let Err(e) = vstar.update(db).await {
        return HttpResponse::InternalServerError().json(e.to_string());
    }

    HttpResponse::Ok().finish()
}

/// 删除star,vstar
#[delete("/api/stars/vstar/{gan_id},{star_id}")]
pub async fn delete_star_vstar(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> impl Responder {
    let db = &app_state.db;
    let gan_id = params.0;
    let star_id = params.1;

    // vstar =f f( (gan, star) )，因此只需查询 gan, star
    // 不存在，返回错误
    let vstar = match gan_star_vstar::Entity::find()
        .filter(gan_star_vstar::Column::GanId.eq(gan_id))
        .filter(gan_star_vstar::Column::StarId.eq(star_id))
        .one(db)
        .await
    {
        Ok(v) => v,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    let vstar = if let Some(vstar) = vstar {
        vstar
    } else {
        return HttpResponse::BadRequest().body("记录不存在");
    };

    if let Err(e) = gan_star_vstar::Entity::delete(vstar.into_active_model())
        .exec(db)
        .await
    {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Ok().finish()
}

/// 生成天干，星曜，仳曜编辑页面
#[get("/stars/vstar/edit")]
pub async fn edit_star_vstar(
    web::Query(r): web::Query<EditStarVstarQuery>,
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    info!("生成编辑星曜化曜页面");
    let db = &app_state.db;
    let mut ctx = tera::Context::new();
    ctx.insert("edit_star_id", &r.star_id);
    ctx.insert("edit_gan_id", &r.gan_id);
    ctx.insert("error_message", "");

    // 查询vstar，用于修改vstar
    match vstar::Entity::find().all(db).await {
        Ok(v) => ctx.insert("vstars", &v),
        Err(e) => {
            ctx.insert("error_message", &e.to_string());
            let s = tmpl
                .render("star/edit_star_vstar.html", &ctx)
                .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
            return Ok(Html(s));
        }
    }

    match gan_star_vstar::Entity::find()
        .left_join(star::Entity)
        .left_join(gan::Entity)
        .left_join(vstar::Entity)
        .select_only()
        .column_as(gan::Column::Id, "gan_id")
        .column_as(star::Column::Id, "star_id")
        .column_as(gan::Column::Name, "gan")
        .column_as(star::Column::Name, "star")
        .column_as(vstar::Column::Name, "vstar")
        .into_model::<GanStarVstarResponser>()
        .all(db)
        .await
    {
        Ok(v) => {
            ctx.insert("star_vstars", &v);
        }
        Err(e) => {
            ctx.insert("error_message", &e.to_string());
        }
    };

    let s = tmpl
        .render("star/edit_star_vstar.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error:{}", e)))?;
    return Ok(Html(s));
}
