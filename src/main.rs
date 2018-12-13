#[macro_use]
extern crate serde_derive;

fn main() {
    println!("{}", "hello world")
}

mod hero;

#[cfg(test)]
mod tests {

    use crate::hero::Hero;
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
