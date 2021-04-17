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
use viewy::components::{Table, Column, VStack, Alignment, Form, TextStyle, Text, Row, Button, ButtonStyle};
use viewy::component::Appendable;

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

struct LoginForm {
    username: String,
    password: String,
}

#[get("/table")]
fn table() -> Html<String> {
    Html(compile_page("Viewy showcase – Login".to_string(), {
        VStack::new(Alignment::Stretch)
            .padding(vec![24])
            .append_child({
                Table::new(vec![
                    Column::new(Some("First col"))
                        .width("50%"),
                    Column::new(Some("Second col"))
                        .width("50%"),
                ])
                    .width("100%")
                    .append_row({
                        Row::new()
                            .append_child({
                                Text::new("hey", TextStyle::Body)
                            })
                            .append_child({
                                Text::new("ho", TextStyle::Body)
                            })
                    })
                    .append_row({
                        Row::new()
                            .append_child({
                                Text::new("hey 2", TextStyle::Body)
                            })
                            .append_child({
                                Text::new("ho 2", TextStyle::Body)
                            })
                    })
            })
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
