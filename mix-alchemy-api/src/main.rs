#[macro_use] extern crate rocket;

mod routes;
mod models;
mod db;

use db::MySqlDatabase;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MySqlDatabase::fairing())
        .mount("/materials", routes![routes::material::get_materials, routes::material::create_material])
        .mount("/mix_designs", routes![routes::mix_design::get_mix_designs, routes::mix_design::create_mix_design])
}
