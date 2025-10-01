#[macro_use]
extern crate rocket;
#[macro_use]
extern crate viewy;
use rayon::prelude::*;
use rocket::tokio::time::interval;
use std::env;
use std::fmt::format;
use std::time::Duration;

use rocket::fs::{FileServer, relative};
use rocket::response::content::{RawCss, RawHtml, RawJavaScript};
use rocket::response::stream::TextStream;
use rocket::serde::uuid::Uuid;
use viewy::bindings::rocket::static_assets::viewy_static_assets_fairing;
use viewy::modifiers::Paddingable;
use viewy::modifiers::actionnable::OnClickActionnable;
use viewy::prelude::actionnable::Action;
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

fn create_button_group(style: ButtonStyle) -> VStack {
    let mut stack = VStack::new(Alignment::Start);
    stack
        .gap(vec![scale(4)])
        .append_child(
            Button::new("Action", style.clone())
                .popup(Popup::new("mypopup").append_child(View::new())),
        )
        .append_child(Button::new("Disabled action", style.clone()).disabled())
        .append_child(Button::new("Destructive action", style.clone()).destructive())
        .append_child(
            Button::new("Disabled destructive action", style)
                .disabled()
                .destructive(),
        );
    stack
}
#[get("/")]
async fn home() -> Page<'static> {
    Page::with_title("Viewy showcase – Home").with_content({
        let mut main_stack = VStack::new(Alignment::Stretch);

        main_stack.append_child(
            Button::new("Open popup", ButtonStyle::Filled).popup(
                Popup::new("mypopup").append_child(create_button_group(ButtonStyle::Filled)),
            ),
        );

        main_stack
            .gap(vec![scale(5)])
            .append_child(create_button_group(ButtonStyle::Filled))
            .append_child(create_button_group(ButtonStyle::Outlined))
            .append_child(create_button_group(ButtonStyle::Flat))
            .append_child(create_button_group(ButtonStyle::Link));

        let mut color_list = VStack::new(Alignment::Stretch);
        color_list.gap(vec![scale(3)]);
        for color in Color::iter() {
            let mut stack = HStack::new(Alignment::Center);
            stack.gap(vec![scale(3)]);

            let mut view = View::new();
            view.width("50px").height("50px").background_color(color);
            stack.append_child(view);

            let mut view = View::new();
            view.text = Some(color.as_str().to_string());
            stack.append_child(view);

            color_list.append_child(stack);
        }
        main_stack.append_child(color_list);
        main_stack
    })
}

#[get("/actions")]
async fn actions() -> Page<'static> {
    Page::with_title("Viewy showcase – Actions").with_content({
        let mut main_stack = VStack::new(Alignment::Stretch);
        let popup_id = Uuid::new_v4();

        main_stack
            .append_child(Button::new("Open popup", ButtonStyle::Filled).on_click(
                Action::OpenPopup {
                    popup_content_url: Some("/actions-popup-content"),
                },
            ));

        main_stack
    })
}
#[get("/actions-popup-content")]
async fn actions_popup_content() -> Page<'static> {
    Page::with_title("Viewy showcase – Actions").with_content({
        let mut main_stack = VStack::new(Alignment::Stretch);
        main_stack
            .padding(vec![scale(5)])
            .append_child(Button::new("Open popup", ButtonStyle::Filled).on_click(
                Action::OpenPopup {
                    popup_content_url: Some("/actions-popup-content"),
                },
            ));

        main_stack
    })
}

#[get("/benchmark")]
fn benchmark() -> Page<'static> {
    Page::with_title("Benchmark viewy").with_content({
        let mut stack = VStack::new(Alignment::Center);
        stack.gap(vec![scale(3)]);
        let buttons: Vec<Node> = (0..50000)
            .map(|i| Button::new(&format!("Button {i}"), ButtonStyle::Filled).into())
            .collect();

        stack.children = buttons;
        stack
    })
}
fn layout(content: Node) -> Node {
    let mut layout = View::new();
    layout.append_child(content);
    layout.text = Some("Le Lorem Ipsum est simplement du faux texte employé dans la composition et la mise en page avant impression. Le Lorem Ipsum est le faux texte standard de l'imprimerie depuis les années 1500, quand un imprimeur anonyme assembla ensemble des morceaux de texte pour réaliser un livre spécimen de polices de texte. Il n'a pas fait que survivre cinq siècles, mais s'est aussi adapté à la bureautique informatique, sans que son contenu n'en soit modifié. Il a été popularisé dans les années 1960 grâce à la vente de feuilles Letraset contenant des passages du Lorem Ipsum, et, plus récemment, par son inclusion dans des applications de mise en page de texte, comme Aldus PageMaker.".to_string());
    layout.into()
}

/*
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
            yield node.render()
        interval.tick().await;
        }
        yield page[1].to_string();
    })
}*/

#[derive(Component, Clone)]
struct MyPage {
    pub btn_nbs: usize,
    pub btn_label: String,
}

impl Component for MyPage {
    fn render(self) -> Node {
        let mut view = View::new();
        for _ in 0..self.btn_nbs {
            view.append_child(Button::new(&self.btn_label, ButtonStyle::Filled));
        }
        view.into()
    }

    fn name() -> &'static str {
        "Boutons"
    }
}

#[get("/component")]
fn component() -> Page<'static> {
    Page::with_title("Test").with_content({
        MyPage {
            btn_nbs: 10,
            btn_label: "Hello".to_string(),
        }
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                get_stylesheet,
                get_scripts,
                home,
                benchmark,
                //hello,
                component,
                actions,
                actions_popup_content
            ],
        )
        .mount("/assets", FileServer::from(relative!("assets")))
        .attach(viewy_static_assets_fairing())
}
