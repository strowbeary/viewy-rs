use crate::layouts;
use rocket::Catcher;
use rocket::http::Status;
use rocket::response::content::RawHtml;
use viewy::components::icons::Lucide;
use viewy::components::*;
use viewy::*;

fn default_catch_page(status: Status) -> RawHtml<String> {
    let mut page_content = VStack::new(Alignment::Center);
    page_content
        .justify_content("center")
        .height("100vh")
        .append_child({ Text::new(format!("{}", status.code).as_str(), TextStyle::LargeTitle) })
        .append_child({ Text::new(format!("{}", status.reason_lossy()).as_str(), TextStyle::H2) })
        .append_child(
            Button::new("Return to home", ButtonStyle::Flat)
                .icon(Lucide::House)
                .margin_top(50)
                .action("/"),
        );

    RawHtml({
        Page::new(
            format!("Viewy showcase – {}", status.reason_lossy()).as_str(),
            layouts::no_layout(),
            VStack::new(Alignment::Center),
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
