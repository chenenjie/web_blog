

#![crate_type = "lib"]
#![crate_name = "e_web"]

#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;
extern crate serde;

use serde::ser::Serialize;
use tera::{Tera, Context};
use std::sync::{Arc, Mutex};
use tera::Error;
use std::collections::HashMap;

lazy_static! {
    pub static ref TEMPLATES: Arc<Mutex<Tera>> = {
        let mut tera = compile_templates!("templates/**/*");
        tera.autoescape_on(vec![".html", ".sql"]);
        // tera.register_filter("do_nothing", do_nothing_filter);
        Arc::new(Mutex::new(tera))
    };
}

pub struct RenderBuilder{
    context: Context,
}

impl RenderBuilder {
    pub fn new() -> RenderBuilder{
        RenderBuilder {
            context: Context::new(),
        }
    }
    pub fn add<T: Serialize>(mut self, key: &str, value: &T) -> RenderBuilder{
        self.context.add(key, value);
        self
    }

    pub fn render(self, template: &str) -> Result<String, Error> {
        let copy = TEMPLATES.clone();
        let tera = copy.lock().unwrap();
        tera.render(template, &self.context)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
