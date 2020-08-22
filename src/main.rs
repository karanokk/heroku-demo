#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::http::RawStr;
use rocket_contrib::serve::StaticFiles;


#[get("/<name>")]
fn root(name: &RawStr) -> &str {
    name.as_str()
}

fn main() {
    rocket::ignite()
        .mount("/public", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .mount("/", routes![root])
        .launch();
}
