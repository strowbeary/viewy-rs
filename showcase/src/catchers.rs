use rocket::response::content::Html;
use rocket::Catcher;
use viewy_rs::Component;
use viewy_rs::components::{VStack, Alignment, Text, TextStyle};
use crate::compile_page;
use viewy_rs::node::DefaultModifiers;

#[catch(404)]
pub fn not_found() -> Html<String> {
    let page = Component::<(), VStack>(|_| {
        let mut o = VStack::new(Alignment::Center)
            .justify_content("center")
            .height("100vh");
        o.add_view_child({
            Text::new("404", TextStyle::LargeTitle)
        });
        o.add_view_child({
            Text::new("Not found", TextStyle::H2)
        });
        o
    });
    let compiled_page = page.compile(());
    Html(compile_page(compiled_page, "auto"))
}

#[catch(403)]
pub fn unauthorized() -> Html<String> {
    let page = Component::<(), VStack>(|_| {
        let mut o = VStack::new(Alignment::Center)
            .justify_content("center")
            .height("100vh");
        o.add_view_child({
            Text::new("403", TextStyle::LargeTitle)
        });
        o.add_view_child({
            Text::new("Unauthorized", TextStyle::H1)
        });
        o
    });
    let compiled_page = page.compile(());
    Html(compile_page(compiled_page, "auto"))
}

pub fn routes() -> Vec<Catcher> {
    catchers![not_found, unauthorized]
}
