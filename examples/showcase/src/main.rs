#[macro_use]
extern crate rocket;
extern crate viewy;

use rocket::response::content::{Css, Html, JavaScript};
use rocket::State;

use viewy::*;
use viewy::components::*;
use crate::catchers::routes;
use viewy::engine::Assets;

mod catchers;
mod components;
mod layouts;
mod pages;

#[get("/app.css")]
fn get_stylesheet(assets: &State<Assets>) -> Css<String> {
    Css(assets.inner().clone().stylesheet)
}

#[get("/app.js")]
fn get_scripts(assets: &State<Assets>) -> JavaScript<String> {
    JavaScript(assets.inner().clone().script)
}

#[get("/")]
fn home() -> Html<String> {
    Html({
        Page::new(
            "Viewy showcase – Home",
            layouts::default_layout,
            pages::home(),
        )
            .compile(RenderMode::Complete)
    })
}

#[get("/login")]
fn login() -> Html<String> {
    Html({
        Page::new(
            "Viewy showcase – Login",
            layouts::login_layout,
            pages::login(),
        )
            .compile(RenderMode::Complete)
    })
}

struct LoginForm {
    username: String,
    password: String,
}


#[get("/table")]
fn table() -> Html<String> {
    Html({
        Page::new(
            "Viewy showcase – Table",
            layouts::default_layout,
            pages::table(),
        )
            .compile(RenderMode::Complete)
    })
}

#[get("/calendar")]
fn calendar() -> Html<String> {
    Html({
        Page::new(
            "Viewy showcase – Calendar",
            layouts::default_layout,
            pages::calendar(),
        )
            .compile(RenderMode::Complete)
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            get_stylesheet,
            get_scripts,
            home,
            login,
            table,
            calendar
        ])
        .register("/" , catchers::routes())
        .manage(Assets::new())

}
