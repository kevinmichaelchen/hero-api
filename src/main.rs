#![feature(proc_macro_hygiene, decl_macro)]

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
use hero::Hero;

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

#[cfg(test)]
mod tests {
    use super::rocket;
    use crate::hero::Hero;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn test_hello() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }

    #[test]
    fn it_works() {
        let hero = Hero {
            id: Some(1),
            name: String::from("Superman"),
            identity: String::from("Clark Kent"),
            hometown: String::from("Metropolis"),
            age: 32,
        };
        let serialized = serde_json::to_string(&hero).unwrap();
        println!("serialized = {}", serialized);
        assert_eq!(serialized, r#"{"id":1,"name":"Superman","identity":"Clark Kent","hometown":"Metropolis","age":32}"#);

        let deserialized: Hero = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);

        assert_eq!(deserialized.id, Some(1));
        assert_eq!(deserialized.name, "Superman");
        assert_eq!(deserialized.identity, "Clark Kent");
        assert_eq!(deserialized.hometown, "Metropolis");
        assert_eq!(deserialized.age, 32);
    }
}
