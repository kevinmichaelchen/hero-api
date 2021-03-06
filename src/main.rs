#![feature(proc_macro_hygiene, decl_macro)]
#![feature(uniform_paths)]
#![feature(result_map_or_else)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate rocket_cors;

use rocket::Rocket;

#[macro_use]
extern crate diesel;
//use rocket_contrib::databases::diesel;

use rocket::http::Method;
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[database("my_db")]
struct MyDatabase(diesel::MysqlConnection);

mod hero;
mod schema;
use hero::{Hero, HeroPatch, HeroWithId};

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
    Hero::update(&conn.0, id, update)
}

#[patch("/<id>", data = "<hero>")]
fn patch(conn: MyDatabase, id: i32, hero: Json<HeroPatch>) -> Json<JsonValue> {
    let update = HeroPatch {
        ..hero.into_inner()
    };
    Hero::patch(&conn.0, id, update)
}

#[delete("/<id>")]
fn delete(conn: MyDatabase, id: i32) -> Json<JsonValue> {
    Json(json!({
        "success": Hero::delete(&conn.0, id)
    }))
}

fn rocket() -> Rocket {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:3000"]);
    assert!(failed_origins.is_empty());
    let options = rocket_cors::Cors {
        allowed_origins,
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Patch,
            Method::Delete,
            Method::Options,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        //        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .mount(
            "/hero",
            routes![create, update, delete, get_bulk, get_detail, patch],
        )
        .mount("/", routes![hello])
        .attach(MyDatabase::fairing())
        .attach(options)
}

fn main() {
    rocket().launch();
}
