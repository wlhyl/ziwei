use entity::entity_enum::{Gan, House, Power, Vstar, WuXing};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct AddGanForm {
    pub name: Gan,
}

#[derive(Deserialize)]
pub struct AddHouseForm {
    // pub id: Option<i32>,
    pub name: House,
}

#[derive(Deserialize)]
pub struct AddVstarForm {
    pub name: Vstar,
}

#[derive(Deserialize)]
pub struct AddPowerForm {
    pub name: Power,
}

#[derive(Deserialize, Validate)]
pub struct AddCategoryForm {
    // pub id: Option<i32>,
    #[validate(length(min = 1), non_control_character)]
    pub name: String,
    #[validate(non_control_character)]
    pub describe: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateCategoryForm {
    // pub id: Option<i32>,
    #[validate(length(min = 1), non_control_character)]
    pub name: String,
    #[validate(non_control_character)]
    pub describe: String,
}

#[derive(Deserialize)]
pub struct EditingCategory {
    pub name_error: Option<bool>,
    pub describe_error: Option<bool>,
    pub name: Option<String>,
    pub describe: Option<String>,
}

#[derive(Deserialize)]
pub struct StarsQuery {
    pub list: Option<String>,
}

#[derive(Deserialize)]
pub struct EditStarForm {
    pub id: Option<i32>,
    // #[validate(length(min = 1), non_control_character)]
    // pub name: String,
    // #[validate(non_control_character)]
    // pub describe: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct StarFormRequest {
    // #[validate(range(min = 0))]
    // pub id: i32,
    #[validate(length(min = 1, message="不能为空"), non_control_character)]
    pub name: String,
    #[validate(range(min = 1, message="最小值为1"))]
    pub category_id: i32,
    pub wu_xing: WuXing,
    #[validate(length(min = 1, message="不能为空"), non_control_character)]
    pub lord: Option<String>,
    #[validate(length(min = 1, message="不能为空"), non_control_character)]
    pub describe: Option<String>, 
}

#[derive(Deserialize, Validate)]
pub struct StarHousePowerRequest {
    #[validate(range(min = 1, message = "必需大于0"))]
    pub star_id: i32,
    #[validate(range(min = 1, message = "必需大于0"))]
    pub house_id: i32,
    #[validate(range(min = 1, message = "必需大于0"))]
    pub power_id: i32,
}

#[derive(Deserialize)]
pub struct EditStarPowerQuery {
    pub star_id: i32,
    pub house_id: i32,
}

#[derive(Deserialize, Validate)]
pub struct GanStarVstarRequest {
    #[validate(range(min = 1, message = "必需大于0"))]
    pub gan_id: i32,
    #[validate(range(min = 1, message = "必需大于0"))]
    pub star_id: i32,
    #[validate(range(min = 1, message = "必需大于0"))]
    pub vstar_id: i32,
}

#[derive(Deserialize)]
pub struct EditStarVstarQuery {
    pub gan_id: i32,
    pub star_id: i32,
}
