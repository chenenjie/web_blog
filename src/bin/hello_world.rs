extern crate iron;
extern crate time;
extern crate e_web;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use serde::ser::Serialize;
use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;
use e_web::RenderBuilder;
use std::collections::HashMap;


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

struct ResponseTime;

impl typemap::Key for ResponseTime {type Value = u64;}

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        Ok(res)
    }
}


fn hello_world(_: &mut Request) -> IronResult<Response> {
    let product = Product::new();

    let render_str = RenderBuilder::new()
        .add("product", &product)
        .add("vat_rate", &0.20)
        .render("hello.html")
        .unwrap_or("render_false".to_string());
    let mut response = Response::with((iron::status::Ok, render_str));
    response.headers.set_raw("Content-Type", vec![b"text/html; charset=UTF-8".to_vec()]);
    Ok(response)
}


fn main() {
    let mut chain = Chain::new(hello_world);
    Iron::new(chain).http("localhost:3000").unwrap();
}
