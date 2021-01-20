#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate viewy_rs;
use viewy_rs::{Component, view};
use viewy_rs::components::*;
use rocket::response::content::Html;
use rocket::response::Content;
use rocket::http::ContentType;
fn compile_page(compiled_page: (String, String, String)) -> Html<String> {
    Html(format!(r"
        <!doctype html>
        <html>
        <head>
            <style>{}</style>
            <script>{}</script>
        </head>
        <body>
            {}
        </body>
        </html>
    ", compiled_page.1, compiled_page.2, compiled_page.0))
}
struct Profil {
    pub name: String,
    pub age: u8
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> Html<String> {
    let profil = Profil {
        name,
        age
    };
    let page = Component::<Profil, Card>(|profil| view! {
        Card(style: CardStyle::Raised) {
            VStack(alignment: HorizontalAlignment::Center) {
                Button(label: profil.name)
                Button(label: profil.age.to_string())
                Button("Goodbye")
                    .action("/goodbye")
            }
        }
    });
    let compiled_page = page.compile(profil);
    compile_page(compiled_page)
}
#[get("/goodbye")]
fn goodbye() -> Html<String> {

    let page = Component::<(), Card>(|profil| view! {
        Card(style: CardStyle::Raised) {
            VStack(alignment: HorizontalAlignment::Center) {
                Button("Hello")
                    .action("/hello/remi/50")
            }
        }
    });
    let compiled_page = page.compile(());
    compile_page(compiled_page)
}

fn main() {
    rocket::ignite().mount("/", routes![hello, goodbye]).launch();
}
