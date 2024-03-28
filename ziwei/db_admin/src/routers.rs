use actix_web::web;

use crate::handlers::{
    category::{add_category, categories, delete_category, edit_category, update_category},
    gan::{add_tian_gan, delete_gan, tan_gan},
    house::{add_house, delete_house, houses},
    index,
    power::{add_power, delete_power, powers},
    star::{
        add_star, add_star_power, add_star_vstar, delete_star, delete_star_power,
        delete_star_vstar, edit_star, edit_star_power, edit_star_vstar, stars, update_star_power,
        update_star_vstar, update_star,
    },
    vstar::{add_vstar, delete_vstar, vstars},
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(tan_gan)
        .service(add_tian_gan)
        .service(delete_gan)
        .service(houses)
        .service(add_house)
        .service(delete_house)
        .service(vstars)
        .service(add_vstar)
        .service(delete_vstar)
        .service(powers)
        .service(add_power)
        .service(delete_power)
        .service(categories)
        .service(add_category)
        .service(delete_category)
        .service(edit_category)
        .service(update_category)
        .service(stars)
        .service(edit_star)
        .service(delete_star)
        .service(add_star)
        .service(update_star)
        .service(add_star_power)
        .service(edit_star_power)
        .service(update_star_power)
        .service(delete_star_power)
        .service(add_star_vstar)
        .service(update_star_vstar)
        .service(delete_star_vstar)
        .service(edit_star_vstar);
}
