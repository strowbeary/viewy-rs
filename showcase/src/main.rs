#![feature(proc_macro_hygiene, decl_macro)]

mod catchers;
mod home;

#[macro_use]
extern crate rocket;
extern crate viewy;

use rocket::response::content::Html;
use viewy::*;

#[get("/")]
fn goodbye() -> Html<String> {
    let page = home::home();
    Html(compile_page(page.compile(), "auto"))
}

fn main() {
    rocket::ignite().mount("/", routes![goodbye]).register(catchers::routes()).launch();
}
