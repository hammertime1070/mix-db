use askama::Template;
use crate::models::{Material, MixDesign}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub materials: Vec<Material>,
    pub mix_designs: Vec<MixDesign>,
}