#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate viewy_rs;
extern crate view;

use rocket::response::content::Html;
use rocket::response::Content;
use rocket::http::ContentType;
use viewy_rs::*;
use viewy_rs::components::*;
use viewy_rs::node::DefaultModifiers;
use view::view;

fn compile_page(compiled_page: (String, String, String)) -> Html<String> {
    Html(format!(r"
        <!doctype html>
        <html>
        <head>
            <title>Viewy-rs showcase</title>
            <style>{}</style>
            <script>{}</script>
        </head>
        <body>
            {}
        </body>
        </html>
    ", compiled_page.1, compiled_page.2, compiled_page.0))
}

struct Profile {
    pub name: String,
    pub age: u8,
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> Html<String> {
    let profil = Profile {
        name,
        age,
    };
    let page = Component::<Profile, Card>(|profil| {
        let mut o = Card::new(CardStyle::Raised);
        o.add_view_child({
            Text::new(profil.name.as_str(), TextStyle::LargeTitle)
        });
        o.add_view_child({
            Text::new(profil.age.to_string().as_str(), TextStyle::Subtitle1)
        });
        o.add_view_child({
            Button::new("Retour", ButtonStyle::Outlined)
                .action("/")
        });
        o
    });
    let compiled_page = page.compile(profil);
    compile_page(compiled_page)
}

#[get("/")]
fn goodbye() -> Html<String> {
    let page = Component::<(), VStack>(|_| {
        let mut o = VStack::new(Alignment::Stretch);
        o.add_view_child({
            TitleBar::new("Viewy-rs showcase")
                .left_item({
                    Button::new("Back", ButtonStyle::Link)
                        .action("/macro")
                        .grid_area("left_item")
                })
        });
        o.add_view_child({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30])
                .margin(vec![30]);
            o.add_view_child({
                Text::new("Ceci est un exemple de page", TextStyle::LargeTitle)
                    .margin_bottom(25)
            });
            o.add_view_child({
                let mut o = VStack::new(Alignment::Center)
                    .gap(vec![16]);
                o.add_view_child({
                    Button::new("Hello", ButtonStyle::Link)
                        .action("/hello/remi/50")
                });
                o.add_view_child({
                    Button::new("Hello", ButtonStyle::Flat)
                        .action("/hello/remi/50")
                });
                o.add_view_child({
                    Button::new("Hello", ButtonStyle::Outlined)
                        .action("/hello/remi/50")
                });
                o.add_view_child({
                    Button::new("Hello", ButtonStyle::Filled)
                        .action("/hello/remi/50")
                });
                o
            });
            o
        });
        o
    });
    let compiled_page = page.compile(());
    compile_page(compiled_page)
}

#[get("/macro")]
fn macro_generated() -> Html<String> {
    let left_item_view: Button = view!{
        Button("Hello", ButtonStyle::Flat)
    };
    let page = Component::<(), VStack>(|_| view!{
        VStack {
            TitleBar("Test macro")
                .left_item(HStack {
                    Button("Left item", ButtonStyle::Link)
                })
            VStack {
                Button("Go to homepage", ButtonStyle::Outlined)
                    .action("/")
            }
                .padding(vec![30])
        }
    });
    let compiled_page = page.compile(());
    compile_page(compiled_page)
}

fn main() {
    rocket::ignite().mount("/", routes![hello, goodbye, macro_generated]).launch();
}
