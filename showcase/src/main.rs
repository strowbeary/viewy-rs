#![feature(proc_macro_hygiene, decl_macro)]

mod catchers;

#[macro_use]
extern crate rocket;
extern crate viewy_rs;
extern crate view;

use rocket::response::content::Html;
use viewy_rs::*;
use viewy_rs::components::*;
use viewy_rs::node::DefaultModifiers;
use view::view;

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
                .bottom_item({
                    Button::new("Back", ButtonStyle::Link)
                        .action("/macro")
                        .grid_area("left_item")
                })
                .right_item({
                    Button::new("Back", ButtonStyle::Link)
                        .action("/macro")
                        .grid_area("left_item")
                })
        });
        o.add_view_child({
            let mut o =
                VStack::new(Alignment::Stretch)
                    .gap(vec![30])
                    .padding(vec![30]);
            o.add_view_child({
                let mut o = Card::new(CardStyle::Raised)
                    .padding(vec![30]);
                o.add_view_child({
                    Text::new("Buttons", TextStyle::LargeTitle)
                        .margin_bottom(25)
                });
                o.add_view_child({
                    let mut o =
                        HStack::new(Alignment::Center)
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
            o.add_view_child({
                let mut o = Card::new(CardStyle::Raised)
                    .padding(vec![30]);
                o.add_view_child({
                    Text::new("Text field", TextStyle::LargeTitle)
                        .margin_bottom(25)
                });
                o.add_view_child({
                    let mut o =
                        VStack::new(Alignment::Stretch)
                            .gap(vec![16]);
                    o.add_view_child({
                        TextField::new("Hello", "text")
                            .placeholder("Optional placeholder")
                    });
                    o.add_view_child({
                        TextField::new("Hello", "text")
                            .label("Label")
                    });
                    o.add_view_child({
                        TextField::new("Hello", "text")
                            .label("Label")
                            .helper_text("Message d'aide")
                    });
                    o
                });
                o
            });
            o.add_view_child({
                let mut o = Card::new(CardStyle::Raised)
                    .padding(vec![30]);
                o.add_view_child({
                    Text::new("Picker", TextStyle::LargeTitle)
                        .margin_bottom(25)
                });
                o.add_view_child({
                    let mut o =
                        VStack::new(Alignment::Stretch)
                            .gap(vec![16]);
                    o.add_view_child({
                        let mut o = Picker::new("Hello", "2", PickerStyle::Segmented)
                            .label("Segmented picker");
                        o.add_view_child({
                            PickerOption::new("Test 1", "1")
                                .icon("user")
                        });
                        o.add_view_child(PickerOption::new("Test 2", "2"));
                        o.add_view_child(PickerOption::new("Test 3", "3"));
                        o
                    });
                    o
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

fn main() {
    rocket::ignite().mount("/", routes![hello, goodbye]).register(catchers::routes()).launch();
}
