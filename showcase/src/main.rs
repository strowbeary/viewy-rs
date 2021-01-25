#![feature(proc_macro_hygiene, decl_macro)]

mod catchers;

#[macro_use]
extern crate rocket;
extern crate viewy_rs;

use rocket::response::content::Html;
use viewy_rs::*;
use viewy_rs::components::*;
use viewy_rs::node::DefaultModifiers;

pub fn compile_page(compiled_page: (String, String, String)) -> Html<String> {
    Html(format!(r"
        <!doctype html>
        <html>
        <head>
            <title>Viewy-rs showcase</title>
            <style>{}</style>
            <script>{}</script>
            <meta charset='utf8' />
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
        o.add_view_child(Box::new({
            Text::new(profil.name.as_str(), TextStyle::LargeTitle)
        }));
        o.add_view_child(Box::new({
            Text::new(profil.age.to_string().as_str(), TextStyle::Subtitle1)
        }));
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
    let page = Component::<(), VStack>(|_| {
        let mut o = VStack::new(Alignment::Stretch);
        o.add_view_child(Box::new({
            TitleBar::new("Viewy-rs showcase")
                .left_item(Box::new({
                    Button::new("Hello", ButtonStyle::Link)
                        .action("/hello/remi/50")
                        .grid_area("left_item")
                }))
        }));
        o.add_view_child(Box::new({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30])
                .margin(vec![30]);
            o.add_view_child(Box::new({
                Text::new("Buttons", TextStyle::LargeTitle)
                    .margin_bottom(25)
            }));
            o.add_view_child(Box::new({
                let mut o = VStack::new(Alignment::Center)
                    .gap(vec![16]);
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
    rocket::ignite().mount("/", routes![hello, goodbye]).register(catchers::routes()).launch();
}
