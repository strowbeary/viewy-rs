use rocket::response::content::RawHtml;
use rocket::{Catcher, Request};
use viewy::components::*;
use viewy::*;
use crate::layouts;
use rocket::http::Status;
use rocket::response::Responder;
use viewy::components::icons::Lucide;

fn default_catch_page(status: Status) -> RawHtml<String>{
    RawHtml({
        Page::new(
            format!("Viewy showcase – {}", status.reason_lossy()).as_str(),
            &layouts::no_layout,
            {
                VStack::new(Alignment::Center)
                    .justify_content("center")
                    .height("100vh")
                    .append_child({
                        Text::new(format!("{}", status.code).as_str(), TextStyle::LargeTitle)
                    })
                    .append_child({
                        Text::new(format!("{}", status.reason_lossy()).as_str(), TextStyle::H2)
                    })
                    .append_child({
                        Button::new("Return to home", ButtonStyle::Flat)
                            .icon(Lucide::Home)
                            .margin_top(50)
                            .action("/")
                    })
            },
        )
            .compile(RenderMode::Complete)
    })
}

#[catch(404)]
fn not_found() -> RawHtml<String> {
    default_catch_page(Status::NotFound)
}

#[catch(403)]
fn forbidden() -> RawHtml<String> {
    default_catch_page(Status::Forbidden)
}

#[catch(500)]
fn server_error() -> RawHtml<String> {
    default_catch_page(Status::InternalServerError)
}

pub fn routes() -> Vec<Catcher> {
    catchers![not_found, forbidden, server_error]
}
