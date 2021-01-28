#![feature(proc_macro_hygiene, decl_macro)]

mod catchers;
mod home;

#[macro_use]
extern crate rocket;
extern crate viewy;
extern crate view;

use rocket::response::content::Html;
use viewy::*;
use viewy::components::VStack;

#[get("/")]
fn goodbye() -> Html<String> {
    let page = Component::<(), VStack>(|_| home::home());
    let compiled_page = page.compile(());
    Html(compile_page(compiled_page, "auto"))
}

fn main() {
    rocket::ignite().mount("/", routes![goodbye]).register(catchers::routes()).launch();
}
