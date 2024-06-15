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
use viewy::widgets::stack::{Alignment, Stack, VStack};

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
        View::new()
            .append_child({
                let mut list = VStack::new(Alignment::Stretch);

                list.append_child({
                    Button::new("Label", ButtonStyle::Filled).popup({
                        Popup::new().append_child({ Button::new("Haha", ButtonStyle::Outlined) })
                    })
                });

                list
            })
            .append_child(Button::new("Label", ButtonStyle::Filled).destructive())
            .append_child({
                let mut color_list = VStack::new(Alignment::Stretch);
                color_list.gap(vec![scale(3)]);
                for color in Color::iter() {
                    color_list.append_child({
                        let mut view = View::new();
                        view.width("100px").height("50px").background_color(color);
                        view.text = Some(color.as_str().to_string());
                        view
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
    // let mut interval = interval(Duration::from_secs(1));
    RawHtml(TextStream! {
        let page_content = Page::with_title("Streaming")
        .with_layout(&layout).compile(RenderMode::LayoutOnly);
        let page = page_content.split("<!--VIEWY_CONTENT-->").collect::<Vec<&str>>();
        yield page[0].to_string();
        //interval.tick().await;
        for i in 0..n {
        let node: Node = Button::new(&format!("Button {i}"), ButtonStyle::Filled).into();
            yield node.clone().into();
        //interval.tick().await;
        }
        //yield page[1].to_string();
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
