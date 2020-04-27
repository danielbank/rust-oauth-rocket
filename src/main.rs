#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use rocket_contrib::serve::StaticFiles;

#[get("/callback?<code>&<state>")]
pub fn auth(code: &RawStr, state: &RawStr) -> String {
    format!("Code = {}, State = {}", code.to_string(), state.to_string())
}

fn main() {
    rocket::ignite()
        .mount("/oauth", routes![auth])
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();
}
