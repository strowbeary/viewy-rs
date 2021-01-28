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
    Html(compile_page(compiled_page, "light"))
}

#[get("/")]
fn goodbye() -> Html<String> {
    let page = Component::<(), VStack>(|_| {
        let mut o = VStack::new(Alignment::Stretch);
        o.add_view_child({
            TitleBar::new("Viewy-rs showcase")
                .left_item({
                    Button::new("Back", ButtonStyle::Link)
                        .icon("arrow-left")
                        .action("/macro")
                        .grid_area("left_item")
                })
                .bottom_item({
                    TextField::new("Search", FieldType::Search)
                        .placeholder("Search for everything")
                        .grid_area("bottom_item")
                })
                .right_item({
                    Button::new("Validate", ButtonStyle::Filled)
                        .icon("check")
                        .grid_area("right_item")
                })
        });

        o.add_view_child({
            let mut o =
                VStack::new(Alignment::Stretch)
                    .gap(vec![20])
                    .padding(vec![30]);

            o.add_view_child({
                let mut o = Card::new(CardStyle::Raised)
                    .padding(vec![30]);
                o.add_view_child({
                    Text::new("Texts", TextStyle::H1)
                        .margin_bottom(25)
                });
                o.add_view_child({
                    let mut o =
                        VStack::new(Alignment::Start)
                            .gap(vec![16])
                            .width("50%");
                    o.add_view_child(Text::new("Large title", TextStyle::LargeTitle));
                    o.add_view_child(Text::new("Title 1", TextStyle::H1));
                    o.add_view_child(Text::new("Subtitle 1", TextStyle::Subtitle1));
                    o.add_view_child(Text::new("Title 2", TextStyle::H2));
                    o.add_view_child(Text::new("Subtitle 2", TextStyle::Subtitle2));
                    o.add_view_child(Text::new("Title 3", TextStyle::H3));
                    o.add_view_child(Text::new("Subtitle 3", TextStyle::Subtitle3));
                    o.add_view_child(Text::new("Headline", TextStyle::Headline));
                    o.add_view_child(Text::new(r#"Body, `Text` component with `Body` text style is compatible with **markdown** annotations.

It is a *long established* fact that a reader will be **distracted** by the readable content of a page when looking at its layout. The point of using Lorem Ipsum is that it has a more-or-less normal distribution of letters, as opposed to using 'Content here, content here', making it look like readable English."#, TextStyle::Body));
                    o.add_view_child(Text::new("Button", TextStyle::Button));
                    o.add_view_child(Text::new("Label", TextStyle::Label));
                    o.add_view_child(Text::new("Overline", TextStyle::Overline));
                    o.add_view_child(Text::new("Caption", TextStyle::Caption));
                    o
                });
                o
            });
            o.add_view_child({
                let mut o = Card::new(CardStyle::Raised)
                    .padding(vec![30]);
                o.add_view_child({
                    Text::new("Buttons", TextStyle::H1)
                        .margin_bottom(25)
                });
                o.add_view_child({
                    let mut o =
                        HStack::new(Alignment::Center)
                            .gap(vec![16]);
                    o.add_view_child({
                        Button::new("Hello", ButtonStyle::Link)
                    });
                    o.add_view_child({
                        Button::new("Hello", ButtonStyle::Flat)
                    });
                    o.add_view_child({
                        Button::new("Hello", ButtonStyle::Outlined)
                    });
                    o.add_view_child({
                        Button::new("Hello", ButtonStyle::Filled)
                    });
                    o
                });
                o.add_view_child({
                    let mut o =
                        HStack::new(Alignment::Center)
                            .gap(vec![16])
                            .margin_top(20);
                    o.add_view_child({
                        Button::new("Valider", ButtonStyle::Link)
                            .icon("check")
                    });
                    o.add_view_child({
                        Button::new("Valider", ButtonStyle::Flat)
                            .icon("check")
                    });
                    o.add_view_child({
                        Button::new("Valider", ButtonStyle::Outlined)
                            .icon("check")
                    });
                    o.add_view_child({
                        Button::new("Valider", ButtonStyle::Filled)
                            .icon("check")
                    });
                    o
                });
                o
            });
            o.add_view_child({
                let mut o = Card::new(CardStyle::Raised)
                    .padding(vec![30]);
                o.add_view_child({
                    Text::new("Text Field", TextStyle::H1)
                        .margin_bottom(25)
                });
                o.add_view_child({
                    let mut o =
                        VStack::new(Alignment::Stretch)
                            .gap(vec![16]);
                    o.add_view_child({
                        TextField::new("input1", FieldType::Text)
                            .placeholder("Optional placeholder")
                    });
                    o.add_view_child({
                        TextField::new("input2", FieldType::Text)
                            .label("Label")
                    });
                    o.add_view_child({
                        TextField::new("input3", FieldType::Text)
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
                    Text::new("Pickers", TextStyle::H1)
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
                    o.add_view_child({
                        let mut o = Picker::new("Hello", "2", PickerStyle::RadioGroup)
                            .label("Radio group picker");
                        o.add_view_child({
                            PickerOption::new("Test 1 - ignored icon", "1")
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
    Html(compile_page(compiled_page, "auto"))
}

fn main() {
    rocket::ignite().mount("/", routes![hello, goodbye]).register(catchers::routes()).launch();
}
