#[macro_use]
extern crate rocket;
extern crate viewy;

use rocket::form::validate::with;
use std::env;
use std::time::Duration;

use rocket::fs::{relative, FileServer};
use rocket::response::content::{RawCss, RawHtml, RawJavaScript};
use rocket::response::stream::TextStream;
use rocket::serde::uuid::Uuid;
use rocket::tokio::time::{interval, Interval};

use viewy::prelude::*;
use viewy::strum::IntoEnumIterator;
use viewy::widgets::stack::{Alignment, HStack, Stack, VStack};

mod dynroutetest;

#[get("/app.css")]
fn get_stylesheet() -> RawCss<String> {
    RawCss(viewy::prelude::get_stylesheet())
}

#[get("/app.js")]
fn get_scripts() -> RawJavaScript<String> {
    RawJavaScript(viewy::prelude::get_scripts())
}

#[get("/")]
async fn home() -> Page<'static> {
    Page::with_title("Viewy showcase – Home").with_content({
        VStack::new(Alignment::Stretch)
            .gap(vec![scale(5)])
            .append_child({
                VStack::new(Alignment::Start).gap(vec![scale(4)])
                    .append_child(Button::new("Action", ButtonStyle::Filled))
                    .append_child(Button::new("Disabled action", ButtonStyle::Filled).disabled())
                    .append_child(Button::new("Destructive action", ButtonStyle::Filled).destructive())
                    .append_child(Button::new("Disabled destructive action", ButtonStyle::Filled).disabled().destructive())

            })
            .append_child({
                VStack::new(Alignment::Start).gap(vec![scale(4)])
                    .append_child(Button::new("Action", ButtonStyle::Outlined))
                    .append_child(Button::new("Disabled action", ButtonStyle::Outlined).disabled())
                    .append_child(Button::new("Destructive action", ButtonStyle::Outlined).destructive())
                    .append_child(Button::new("Disabled destructive action", ButtonStyle::Outlined).disabled().destructive())

            })
            .append_child({
                VStack::new(Alignment::Start).gap(vec![scale(4)])
                    .append_child(Button::new("Action", ButtonStyle::Flat))
                    .append_child(Button::new("Disabled action", ButtonStyle::Flat).disabled())
                    .append_child(Button::new("Destructive action", ButtonStyle::Flat).destructive())
                    .append_child(Button::new("Disabled destructive action", ButtonStyle::Flat).disabled().destructive())
            })
            .append_child({
                VStack::new(Alignment::Start).gap(vec![scale(4)])
                    .append_child(Button::new("Action", ButtonStyle::Link))
                    .append_child(Button::new("Disabled action", ButtonStyle::Link).disabled())
                    .append_child(Button::new("Destructive action", ButtonStyle::Link).destructive())
                    .append_child(Button::new("Disabled destructive action", ButtonStyle::Link).disabled().destructive())
            })
            .append_child({
                let mut color_list = VStack::new(Alignment::Stretch);
                color_list.gap(vec![scale(3)]);
                for color in Color::iter() {
                    color_list.append_child({
                        HStack::new(Alignment::Center)
                            .gap(vec![scale(3)])
                            .append_child({
                                let mut view = View::new();
                                view.width("50px").height("50px").background_color(color);
                                view
                            })
                            .append_child({
                                let mut view = View::new();
                                view.text = Some(color.as_str().to_string());
                                view
                            })

                    });
                }
                color_list
            })
    })
}

#[get("/benchmark")]
fn benchmark() -> Page<'static> {
    Page::with_title("Benchmark viewy").with_content({
        let mut stack = VStack::new(Alignment::Center);
        stack.gap(vec![scale(3)]);
        for i in 0..50000 {
            stack.append_child({ Button::new(&format!("Button {i}"), ButtonStyle::Filled) });
        }
        stack
    })
}
fn layout(content: Node) -> Node {
    let mut layout = View::new();
    layout.append_child(content);
    layout.text = Some("Le Lorem Ipsum est simplement du faux texte employé dans la composition et la mise en page avant impression. Le Lorem Ipsum est le faux texte standard de l'imprimerie depuis les années 1500, quand un imprimeur anonyme assembla ensemble des morceaux de texte pour réaliser un livre spécimen de polices de texte. Il n'a pas fait que survivre cinq siècles, mais s'est aussi adapté à la bureautique informatique, sans que son contenu n'en soit modifié. Il a été popularisé dans les années 1960 grâce à la vente de feuilles Letraset contenant des passages du Lorem Ipsum, et, plus récemment, par son inclusion dans des applications de mise en page de texte, comme Aldus PageMaker.".to_string());
    layout.into()
}
#[get("/infinite-hellos?<n>")]
fn hello(n: usize) -> RawHtml<TextStream![String]> {
     let mut interval = interval(Duration::from_millis(10));
    RawHtml(TextStream! {
        let page_content = Page::with_title("Streaming")
        .with_layout(&layout).compile(RenderMode::LayoutOnly);
        let page = page_content.split("<!--VIEWY_CONTENT-->").collect::<Vec<&str>>();
        yield page[0].to_string();
        interval.tick().await;
        for i in 0..n {
        let node: Node = Button::new(&format!("Button {i}"), ButtonStyle::Filled).into();
            yield <Node as Into<HtmlCssJs>>::into(node).html;
        interval.tick().await;
        }
        yield page[1].to_string();
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![get_stylesheet, get_scripts, home, benchmark, hello],
        )
        .mount("/assets", FileServer::from(relative!("assets")))
}
