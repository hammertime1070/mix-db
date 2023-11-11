use diesel::prelude::*;
use crate::templates::IndexTemplate;

#[get("/")]
pub fn index() -> IndexTemplate {
    
    IndexTemplate {
        // Initialize template data
        materials,
        mix_designs,
    }
}
