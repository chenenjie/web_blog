#[macro_use]
extern crate serde_derive;

extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Pet {
    name: String,
}

fn main() {
    let pet = Pet { name: String::from("Ferris") };

    let serialized = serde_json::to_string(&pet).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Pet = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}