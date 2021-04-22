use rocket::response::content::Html;
use rocket::Catcher;
use viewy::components::*;
use viewy::*;
use viewy::page::{Page, RenderMode};

#[catch(404)]
pub fn not_found() -> Html<String> {
    Html({
        Page::new("Viewy showcase – Page not found", {
            VStack::new(Alignment::Center)
                .justify_content("center")
                .height("100vh")
                .append_child({
                    Text::new("404", TextStyle::LargeTitle)
                })
                .append_child({
                    Text::new("Not found", TextStyle::H2)
                })
        })
            .compile(RenderMode::Complete)
    })
}

#[catch(403)]
pub fn unauthorized() -> Html<String> {
    Html({
        Page::new("Viewy showcase – Not authorized", {
            VStack::new(Alignment::Center)
                .justify_content("center")
                .height("100vh")
                .append_child({
                    Text::new("403", TextStyle::LargeTitle)
                })
                .append_child({
                    Text::new("Unauthorized", TextStyle::H1)
                })
        })
            .compile(RenderMode::Complete)
    })
}

pub fn routes() -> Vec<Catcher> {
    catchers![not_found, unauthorized]
}
