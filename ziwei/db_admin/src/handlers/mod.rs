pub mod category;
pub mod house;
pub mod power;
pub mod vstar;
pub mod star;
pub mod gan;

use actix_web::{
    error, get,
    web::{self},
    Error, Responder,
};
use actix_web_lab::respond::Html;

#[get("/")]
pub async fn index(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    let s = tmpl
        .render("index.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(Html(s))
}
