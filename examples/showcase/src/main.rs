#[macro_use]
extern crate rocket;
extern crate viewy;

use rocket::response::content::{Css, Html, JavaScript};
use rocket::State;

use viewy::*;
use viewy::components::*;
use crate::catchers::routes;
use viewy::engine::Assets;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

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
            &layouts::default_layout,
            pages::home()
                .append_child({
                    Snackbar::new("Une erreur est survenue")
                }),
        )
            .compile(RenderMode::Complete)
    })
}

#[get("/login")]
fn login() -> Html<String> {
    Html({
        Page::new(
            "Viewy showcase – Login",
            &layouts::login_layout,
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
            &layouts::default_layout,
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
            &layouts::default_layout,
            pages::calendar(),
        )
            .compile(RenderMode::Complete)
    })
}

#[get("/menus")]
fn menus() -> Html<String> {
    Html({
        Page::new(
            "Viewy showcase – Navigation & menus",
            &layouts::default_layout,
            pages::navigation_and_menus(),
        )
            .compile(RenderMode::Complete)
    })
}

#[get("/tabs")]
fn tabs() -> Html<String> {
    Html({
        Page::new(
            "Viewy showcase – Tab view",
            &layouts::default_layout,
            pages::tabs(),
        )
            .compile(RenderMode::Complete)
    })
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SearchResult<T: Serialize> {
    pub label: String,
    pub value: T
}

#[get("/search?<q>")]
fn search(q: String) -> Json<Vec<SearchResult<String>>> {
    Json(vec![
        SearchResult {
            label: q,
            value: "test1".to_string()
        },
        SearchResult {
            label: "Test 3".to_string(),
            value: "test3".to_string()
        },
        SearchResult {
            label: "Test 2".to_string(),
            value: "test2".to_string()
        }
    ])
}

#[get("/search")]
fn search_page() -> Html<String> {
    Html({
        Page::new(
            "Viewy showcase – Search demo",
            &layouts::default_layout,
            pages::searchable_input_page(),
        )
            .compile(RenderMode::Complete)
    })
}

#[get("/signature")]
fn signature() -> Html<String> {
    Html({
        Page::new(
            "Viewy showcase – Signature field",
            &layouts::default_layout,
            pages::signature_field(),
        )
            .compile(RenderMode::Complete)
    })
}
#[get("/forms")]
fn forms() -> Html<String> {
    Html({
        Page::new(
            "Viewy showcase – Forms",
            &layouts::default_layout,
            pages::forms(),
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
            calendar,
            menus,
            search_page,
            search,
            signature,
            forms,
            tabs
        ])
        .register("/", catchers::routes())
        .manage(Assets::new())
}
