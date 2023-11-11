use diesel::prelude::*;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use crate::database;
use crate::models::*;
use crate::schema::{materials, mix_designs, match_mix_materials};

// List all materials
#[get("/materials")]
pub fn list_materials() -> Json<Vec<Material>> {
    let connection = &mut database::establish_connection();
    materials::table
        .load::<Material>(connection)
        .map(Json)
        .expect("Error loading materials")
}

// Add a new material
#[post("/materials", data = "<new_material>")]
pub fn add_material(new_material: Json<NewMaterial>) -> Json<Material> {
    let connection = &mut database::establish_connection();
    diesel::insert_into(materials::table)
        .values(new_material.into_inner())
        .execute(connection)
        .expect("Error adding material");
    
    Json(materials::table.order(materials::id.desc()).first(connection).unwrap())
}

// List all mix designs
#[get("/mix_designs")]
pub fn list_mix_designs() -> Json<Vec<MixDesign>> {
    let connection = &mut database::establish_connection();
    mix_designs::table.load::<MixDesign>(connection).map(Json).expect("Error loading mix designs")
}

// Add a new mix design
#[post("/mix_designs", data = "<new_mix_design>")]
pub fn create_mix_design(new_mix_design: Json<NewMixDesign>) -> Json<MixDesign> {
    let connection = &mut database::establish_connection();
    diesel::insert_into(mix_designs::table)
        .values(new_mix_design.into_inner())
        .execute(connection)
        .expect("Error creating new mix design");

    Json(mix_designs::table
        .order(mix_designs::id.desc())
        .first(connection).unwrap()
    )
}

// Add Material to Mix
#[post("/mix_materials", data = "<new_mix_material>")]
pub fn add_material_to_mix(new_mix_material: Json<NewMatchMixMaterial>) -> Json<MatchMixMaterial> {
    let connection = &mut database::establish_connection();
    diesel::insert_into(match_mix_materials::table)
        .values(new_mix_material.into_inner())
        .execute(connection)
        .expect("Error adding material to mix design");

    Json(match_mix_materials::table
        .order((match_mix_materials::mix_id.desc(), match_mix_materials::material_id.desc()))
        .first(connection).unwrap()
    )
}

// View Components of a Mix Design
#[get("/mix_designs/<mix_id>/materials")]
pub fn get_mix_materials(mix_id: i32) -> Json<Vec<MatchMixMaterial>> {
    let connection = &mut database::establish_connection();
    match_mix_materials::table
        .filter(match_mix_materials::mix_id.eq(mix_id))
        .load::<MatchMixMaterial>(connection)
        .map(Json)
        .expect("Error loading mix materials")
}

// Endpoint for Pivot Table Creation
#[get("/pivot_data")]
pub fn get_pivot_data() -> Json<Vec<(MixDesign, Vec<MatchMixMaterial>)>> {
    let connection = &mut database::establish_connection();

    // Fetch all mix designs
    let mixes = mix_designs::table
        .load::<MixDesign>(connection)
        .expect("Error loading mix designs");

    // For each mix design, fetch associated materials and quantities
    let mix_data = mixes.into_iter().map(|mix| {
        let materials = match_mix_materials::table
            .filter(match_mix_materials::mix_id.eq(mix.id))
            .load::<MatchMixMaterial>(connection)
            .expect("Error loading mix materials for mix design");

        (mix, materials)
    }).collect::<Vec<_>>();

    Json(mix_data)
}
