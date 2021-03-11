use rocket::response::content::Html;
use rocket::Catcher;
use viewy::components::{VStack, Alignment, Text, TextStyle};
use crate::compile_page;
use viewy::{DefaultModifiers, ToHtml};

#[catch(404)]
pub fn not_found() -> Html<String> {
    let mut page = VStack::new(Alignment::Center)
        .justify_content("center")
        .height("100vh");
    page.append_child({
        Text::new("404", TextStyle::LargeTitle)
    });
    page.append_child({
        Text::new("Not found", TextStyle::H2)
    });
    let compiled_page = page.compile();
    Html(compile_page(compiled_page, "auto"))
}

#[catch(403)]
pub fn unauthorized() -> Html<String> {
    let mut page = VStack::new(Alignment::Center)
        .justify_content("center")
        .height("100vh");
    page.append_child({
        Text::new("403", TextStyle::LargeTitle)
    });
    page.append_child({
        Text::new("Unauthorized", TextStyle::H1)
    });
    let compiled_page = page.compile();
    Html(compile_page(compiled_page, "auto"))
}

pub fn routes() -> Vec<Catcher> {
    catchers![not_found, unauthorized]
}
