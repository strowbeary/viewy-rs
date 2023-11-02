#[macro_use]
extern crate rocket;
extern crate viewy;

use std::env;
use rocket::fs::{FileServer, relative};
use rocket::response::content::{RawCss, RawJavaScript};
use viewy::prelude::*;


#[get("/app.css")]
fn get_stylesheet() -> RawCss<String> {
    RawCss(viewy::prelude::get_stylesheet())
}

#[get("/app.js")]
fn get_scripts() -> RawJavaScript<String> {
    RawJavaScript(String::new())
}

#[get("/")]
fn home() -> Page<'static> {
    Page::with_title("Viewy showcase – Home")
        .with_content({
            View::new()
                .append_child(
                    Button::new("Label", ButtonStyle::Filled)
                        .popup({
                            Popup::new()
                                .append_child({
                                    Button::new("Haha", ButtonStyle::Outlined)
                                })
                        })
                )
                .append_child(
                    Button::new("Label", ButtonStyle::Filled)
                        .destructive()
                )
                .append_child({
                   let mut color_list = View::new();
                    for color in Color::iter() {
                        color_list.append_child({
                            let mut view = View::new();
                            view.width("100px")
                                .height("50px")
                                .background_color(*color);
                            view.text = Some(color.as_str().to_string());
                            view
                        });
                    }
                    color_list
                })
        })
}

struct LoginForm {
    username: String,
    password: String,
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            get_stylesheet,
            get_scripts,
            home,

        ])
        .mount("/assets", FileServer::from(relative!("assets")))
}
