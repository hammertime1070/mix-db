use diesel::prelude::Queryable;
use rocket::serde::Serialize;
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Material {
    pub id: i32,
    pub category: String,
    pub name: String,
    pub price: BigDecimal,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct MixDesign {
    pub id: i32,
    pub location: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct MatchMixMaterial {
    pub mix_id: i32,
    pub material_id: i32,
    pub quantity: BigDecimal,
}

// Insertable structure for creating new material entries
#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = materials)]
pub struct NewMaterial {
    pub category: String,
    pub name: String,
    pub price: BigDecimal,
    // `created_at` is not included here as it is set by default in the database
}

// Insertable structure for creating new mix design entries
#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = mix_designs)]
pub struct NewMixDesign {
    pub location: String,
    pub name: String,
    pub description: Option<String>,
    // `created_at` is not included here as it is set by default in the database
}

// Insertable structure for creating new match mix material entries
#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = match_mix_materials)]
pub struct NewMatchMixMaterial {
    pub mix_id: i32,
    pub material_id: i32,
    pub quantity: BigDecimal,
}

