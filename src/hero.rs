use super::schema::hero as hero_table;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug, Queryable, Insertable)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}

impl Hero {
    pub fn create(connection: &diesel::MysqlConnection, hero: Json<Hero>) -> Json<Hero> {
        diesel::insert_into(hero_table::table)
            .values(&hero)
            .execute(connection)
            .expect("Error creating new hero");

        hero_table::table
            .order(hero_table::id.desc())
            .first(connection)
            .unwrap()
    }
}
