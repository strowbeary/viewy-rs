#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate viewy;

mod catchers;
mod home;
mod login;

use rocket::response::content::{Html, Css, JavaScript};
use viewy::*;
use rocket::State;

#[get("/app.css")]
fn get_stylesheet(assets: State<Assets>) -> Css<String> {
    Css(assets.inner().clone().stylesheet)
}

#[get("/app.js")]
fn get_scripts(assets: State<Assets>) -> JavaScript<String> {
    JavaScript(assets.inner().clone().script)
}

#[get("/")]
fn home() -> Html<String> {
    let page = home::home();
    Html(compile_page(page.compile(), "auto"))
}

#[get("/login")]
fn login() -> Html<String> {
    let page = login::login();
    Html(compile_page(page.compile(), "auto"))
}

fn main() {
    rocket::
    ignite()
        .mount("/", routes![
        get_stylesheet,
        get_scripts,
        home,
        login
    ])
        .register(catchers::routes())
        .manage(Assets::new())
        .launch();
}
