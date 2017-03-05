#[macro_use]
extern crate tera;
extern crate glob;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use serde_json::value::{Value, to_value};
use tera::{Tera, Context};
use glob::glob;


fn glob_test(){
    for entry in glob("src/bin/tt/*").expect("Failed to read glob pattern") {
        match entry{
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => print!("{:?}", e),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    name: String,
    manufacturer: String,
    summary: String,
    price: i32,
}

impl Product {
    fn new() -> Product{
        Product {
            name: "fuck".to_string(),
            manufacturer: "fuck manu".to_string(),
            summary: "summary".to_string(),
            price: 1i32,
        }
    }
}

fn main() {
    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let product = Product {
        name: "fuck".to_string(),
        manufacturer: "fuck manu".to_string(),
        summary: "summary".to_string(),
        price: 1i32,
    };

    let mut context = Context::new();
    context.add("product", &product);
    context.add("vat_rate", &0.20);

    println!("{:?}", tera);

    match tera.render("templates/hello.html", context) {
        Ok(s) => println!("{:?}", s),
        Err(e) => {
            println!("Error: {}", e);
            for e in e.iter().skip(1) {
                println!("Reason: {}", e);
            }
        }
    };    
}