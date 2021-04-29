use rocket::response::content::Html;
use rocket::{Catcher, Request};
use viewy::components::*;
use viewy::*;
use crate::layouts;
use rocket::http::Status;
use rocket::response::Responder;

fn default_catch_page(status: Status) -> Html<String>{
    Html({
        Page::new(
            format!("Viewy showcase – {}", status.reason).as_str(),
            layouts::no_layout,
            {
                VStack::new(Alignment::Center)
                    .justify_content("center")
                    .height("100vh")
                    .append_child({
                        Text::new(format!("{}", status.code).as_str(), TextStyle::LargeTitle)
                    })
                    .append_child({
                        Text::new(format!("{}", status.reason).as_str(), TextStyle::H2)
                    })
                    .append_child({
                        Button::new("Return to home", ButtonStyle::Flat)
                            .icon("home")
                            .margin_top(50)
                            .action("/")
                    })
            },
        )
            .compile(RenderMode::Complete)
    })
}

#[catch(404)]
fn not_found() -> Html<String> {
    default_catch_page(Status::NotFound)
}

#[catch(403)]
fn forbidden() -> Html<String> {
    default_catch_page(Status::Forbidden)
}

#[catch(500)]
fn server_error() -> Html<String> {
    default_catch_page(Status::InternalServerError)
}

pub fn routes() -> Vec<Catcher> {
    catchers![not_found, forbidden, server_error]
}
