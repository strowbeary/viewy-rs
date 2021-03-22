#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate viewy;

mod catchers;
mod home;
mod login;

use rocket::response::content::Html;
use viewy::*;

#[get("/")]
fn home() -> Html<String> {
    let page = home::home();
    Html(compile_page(page.compile(), "dark"))
}
#[get("/login")]
fn login() -> Html<String> {
    let page = login::login();
    Html(compile_page(page.compile(), "light"))
}

fn main() {
    rocket::ignite().mount("/", routes![home, login]).register(catchers::routes()).launch();
}
