use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}

impl Hero {
    pub fn create(conn: &diesel::MysqlConnection, hero: Json<Hero>) -> Json<Hero> {
        hero
    }
}
