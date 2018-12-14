#![feature(proc_macro_hygiene, decl_macro)]
#![feature(uniform_paths)]
#![feature(result_map_or_else)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

use rocket::Rocket;

#[macro_use]
extern crate diesel;
//use rocket_contrib::databases::diesel;

use rocket_contrib::json::{Json, JsonValue};

#[database("my_db")]
struct MyDatabase(diesel::MysqlConnection);

mod hero;
mod schema;
use hero::{Hero, HeroWithId};

#[cfg(test)]
mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<hero>")]
fn create(conn: MyDatabase, hero: Json<Hero>) -> Json<JsonValue> {
    let insert = Hero {
        ..hero.into_inner()
    };
    Hero::create(&conn.0, &insert)
}

#[get("/")]
fn get_bulk(conn: MyDatabase) -> Json<JsonValue> {
    Json(json!(Hero::get_bulk(&conn.0)))
}

#[get("/<id>")]
fn get_detail(conn: MyDatabase, id: i32) -> Json<JsonValue> {
    Hero::get_detail(&conn.0, id)
}

#[put("/<id>", data = "<hero>")]
fn update(conn: MyDatabase, id: i32, hero: Json<HeroWithId>) -> Json<JsonValue> {
    // TODO should they be updating *with* ID?
    let update = HeroWithId {
        ..hero.into_inner()
    };
    Json(json!({
        "success": Hero::update(&conn.0, id, update)
    }))
}

#[delete("/<id>")]
fn delete(conn: MyDatabase, id: i32) -> Json<JsonValue> {
    Json(json!({
        "success": Hero::delete(&conn.0, id)
    }))
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount(
            "/hero",
            routes![create, update, delete, get_bulk, get_detail],
        )
        .mount("/", routes![hello])
        .attach(MyDatabase::fairing())
}

fn main() {
    rocket().launch();
}
