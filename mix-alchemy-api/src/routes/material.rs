use rocket::serde::json::Json;
use crate::db::MySqlDatabase;

// This assumes you have a Material struct defined somewhere in your models module.
use crate::models::Material;

#[get("/")]
pub async fn get_materials(_conn: MySqlDatabase) -> Json<Vec<Material>> {
    // For now, return an empty list.
    Json(vec![])
}

#[post("/", format = "json", data = "<material>")]
pub async fn create_material(_conn: MySqlDatabase, material: Json<Material>) -> &'static str {
    // For now, just acknowledge receipt.
    "Material received"
}