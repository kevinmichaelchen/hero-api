use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use rocket_contrib::json::Json;

use super::schema;
use schema::hero;

#[derive(AsChangeset, Serialize, Deserialize, Debug, Queryable, Insertable)]
#[table_name = "hero"]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}

impl Hero {
    pub fn create(connection: &diesel::MysqlConnection, h: &Hero) -> Hero {
        diesel::insert_into(hero::table)
            .values(h)
            .execute(connection)
            .expect("Error creating new hero");

        // TODO return record we just inserted instead of mock
        Hero {
            id: Some(1),
            name: String::from("Superman"),
            identity: String::from("Clark Kent"),
            hometown: String::from("Metropolis"),
            age: 32,
        }
    }
}
