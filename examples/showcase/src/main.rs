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
use viewy::components::{Table, Column};

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
    Html(compile_page("Viewy showcase – Home".to_string(), page.compile(), "auto"))
}

#[get("/login")]
fn login() -> Html<String> {
    let page = login::login();
    Html(compile_page("Viewy showcase – Login".to_string(), page.compile(), "auto"))
}

#[get("/table")]
fn table() -> Html<String> {
    Html(compile_page("Viewy showcase – Login".to_string(), {
        Table::new(vec![
            Column::new("First col"),
            Column::new("Second col"),
        ])
            .rows(vec![

            ])
            .width("100%")
    }.compile(), "auto"))
}

fn main() {
    rocket::
    ignite()
        .mount("/", routes![
        get_stylesheet,
        get_scripts,
        home,
        login,
            table
    ])
        .register(catchers::routes())
        .manage(Assets::new())
        .launch();
}
