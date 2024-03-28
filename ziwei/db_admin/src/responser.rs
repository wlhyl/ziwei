use entity::entity_enum::{Gan, House, Power, Vstar};
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(FromQueryResult, Serialize)]
pub struct StarHousePowerResponser {
    pub star_id: i32,
    pub house_id: i32,
    pub star: String,
    pub house: House,
    pub power: Power,
}

#[derive(FromQueryResult, Serialize)]
pub struct GanStarVstarResponser {
    pub gan_id: i32,
    pub star_id: i32,
    pub gan: Gan,
    pub star: String,
    pub vstar: Vstar,
}
