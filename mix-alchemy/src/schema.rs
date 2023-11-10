// @generated automatically by Diesel CLI.

diesel::table! {
    match_mix_materials (mix_id, material_id) {
        mix_id -> Integer,
        material_id -> Integer,
        quantity -> Decimal,
    }
}

diesel::table! {
    materials (id) {
        id -> Integer,
        #[max_length = 255]
        category -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        price -> Decimal,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    mix_designs (id) {
        id -> Integer,
        #[max_length = 255]
        location -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    match_mix_materials,
    materials,
    mix_designs,
);
