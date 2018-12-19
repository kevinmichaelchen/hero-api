use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use rocket::response::status;
use rocket_contrib::json::{Json, JsonValue};

use super::schema;
use schema::hero;

// gotta have 2 structs, one for input, one for persistence: https://github.com/diesel-rs/diesel/issues/1440#issuecomment-354573185
#[derive(AsChangeset, Serialize, Deserialize, Debug, Queryable)]
#[table_name = "hero"]
pub struct HeroWithId {
    pub id: i32,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
    pub image_url: Option<String>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Insertable)]
#[table_name = "hero"]
pub struct Hero {
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
    pub image_url: Option<String>,
}

fn get_status_code_from_diesel_err(e: DieselError) -> i32 {
    if e == DieselError::NotFound {
        404
    } else {
        400
    }
}

fn diesel_err_to_json(e: DieselError) -> Json<JsonValue> {
    Json(json!({
        "error": e.to_string(),
        "status_code": get_status_code_from_diesel_err(e),
    }))
}

impl Hero {
    fn get_most_recently_created_hero(connection: &diesel::MysqlConnection) -> HeroWithId {
        hero::table
            .order(hero::id.desc())
            .first(connection)
            .unwrap()
    }

    fn find_by_id(connection: &MysqlConnection, id: i32) -> Result<HeroWithId, DieselError> {
        hero::table.find(id).first(connection)
    }

    pub fn create(connection: &diesel::MysqlConnection, h: &Hero) -> Json<JsonValue> {
        diesel::insert_into(hero::table)
            .values(h)
            .execute(connection)
            .map_or_else(
                |e| diesel_err_to_json(e),
                |res| Json(json!(Hero::get_most_recently_created_hero(connection))),
            )
    }

    pub fn get_bulk(connection: &MysqlConnection) -> Vec<HeroWithId> {
        hero::table
            .order(hero::id)
            .load::<HeroWithId>(connection)
            .unwrap()
    }

    pub fn get_detail(connection: &MysqlConnection, id: i32) -> Json<JsonValue> {
        Hero::find_by_id(connection, id).map_or_else(|e| diesel_err_to_json(e), |h| Json(json!(h)))
    }

    pub fn update(connection: &MysqlConnection, id: i32, h: HeroWithId) -> bool {
        diesel::update(hero::table.find(id))
            .set(&h)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(connection: &MysqlConnection, id: i32) -> bool {
        diesel::delete(hero::table.find(id))
            .execute(connection)
            .is_ok()
    }
}
