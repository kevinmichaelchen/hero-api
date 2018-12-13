#![feature(proc_macro_hygiene, decl_macro)]
#![feature(uniform_paths)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

use rocket::Rocket;
use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;

#[database("my_db")]
struct MyDatabase(diesel::MysqlConnection);

mod hero;
mod schema;
use hero::Hero;

#[cfg(test)]
mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<hero>")]
fn create(conn: MyDatabase, hero: Json<Hero>) -> Json<Hero> {
    Hero::create(&conn.0, hero)
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/hero", routes![create])
        .mount("/", routes![hello])
        .attach(MyDatabase::fairing())
}

fn main() {
    rocket().launch();
}
