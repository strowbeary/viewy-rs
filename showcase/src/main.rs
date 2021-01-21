#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate viewy_rs;
use rocket::response::content::Html;
use rocket::response::Content;
use rocket::http::ContentType;
use viewy_rs::*;
use viewy_rs::components::*;

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
    pub age: u8,
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> Html<String> {
    let profil = Profil {
        name,
        age,
    };
    let page = Component::<Profil, Card>(|profil| {
        let mut o = Card::new(CardStyle::Raised);
        o.add_view_child(Box::new({
            Button::new("Retour", ButtonStyle::Outlined)
                .action("/")
        }));
        o
    });
    let compiled_page = page.compile(profil);
    compile_page(compiled_page)
}

#[get("/")]
fn goodbye() -> Html<String> {
    let page = Component::<(), VStack>(|profil| {
        let mut o = VStack::new(HorizontalAlignment::Center)
            .padding(vec![30]);
        o.add_view_child(Box::new({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30]);
            o.add_view_child(Box::new({
                Text::new("Ceci est un exemple de page", TextStyle::LargeTitle)
                    .margin_bottom(25)
            }));
            o.add_view_child(Box::new({
                let mut o = VStack::new(HorizontalAlignment::Center)
                    .gap(16);
                o.add_view_child(Box::new({
                    Button::new("Hello", ButtonStyle::Link)
                        .action("/hello/remi/50")
                }));
                o.add_view_child(Box::new({
                    Button::new("Hello", ButtonStyle::Flat)
                        .action("/hello/remi/50")
                }));
                o.add_view_child(Box::new({
                    Button::new("Hello", ButtonStyle::Outlined)
                        .action("/hello/remi/50")
                }));
                o.add_view_child(Box::new({
                    Button::new("Hello", ButtonStyle::Filled)
                        .action("/hello/remi/50")
                }));
                o
            }));
            o
        }));
        o
    });
    let compiled_page = page.compile(());
    compile_page(compiled_page)
}

fn main() {
    rocket::ignite().mount("/", routes![hello, goodbye]).launch();
}
