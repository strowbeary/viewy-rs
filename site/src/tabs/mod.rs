use viewy::bindings::uri::Uri;
use viewy::prelude::*;
use viewy::widgets::tabs::{Tab, TabContainer};

#[get("/tabs")]
pub fn tabs() -> Page<'static> {
    Page::with_title("Test").with_content({
        let mut tab_container = TabContainer::new();
        tab_container.add_tab(Tab::new("Overview", Uri::from(uri!(tab1()))));
        tab_container.add_tab(Tab::new("Projects", Uri::from(uri!(tab2()))));
        tab_container.add_tab(Tab::new("Account", Uri::from(uri!(tab3()))));
        tab_container
    })
}

#[get("/tab1")]
pub async fn tab1() -> Page<'static> {
    Page::with_title("Tab 1")
        .with_content({
            let mut main_stack = VStack::new(Alignment::Stretch);
            main_stack
                .gap(vec![scale(4)])
                .padding(vec![scale(5)])
                .append_child(Text::new("Tab 1", TextStyle::H1))
                .append_child(Text::new("Text", TextStyle::Body))
                .append_child(
                    HStack::new(Alignment::Center)
                        .gap(vec![scale(4)])
                        .append_child(Button::new("Cancel", ButtonStyle::Outlined))
                        .append_child(Button::new("Ok", ButtonStyle::Filled)),
                );

            main_stack
        })
}

#[get("/tab2")]
pub async fn tab2() -> Page<'static> {
    Page::with_title("Tab 2")
        .with_content({
            let mut main_stack = VStack::new(Alignment::Stretch);
            main_stack
                .gap(vec![scale(4)])
                .padding(vec![scale(5)])
                .append_child(Text::new("Tab 2", TextStyle::H1))
                .append_child(Text::new("Text", TextStyle::Body))
                .append_child(
                    HStack::new(Alignment::Center)
                        .gap(vec![scale(4)])
                        .append_child(Button::new("Ok", ButtonStyle::Filled)),
                );

            main_stack
        })
}
#[get("/tab3")]
pub async fn tab3() -> Page<'static> {
    Page::with_title("Tab 3")
        .with_content({
            let mut main_stack = VStack::new(Alignment::Stretch);
            main_stack
                .gap(vec![scale(4)])
                .padding(vec![scale(5)])
                .append_child(Text::new("Tab 3", TextStyle::H1))
                .append_child(Text::new("Text", TextStyle::Body))
                .append_child(
                    HStack::new(Alignment::Center)
                        .gap(vec![scale(4)])
                        .append_child(Button::new("Continue", ButtonStyle::Filled)),
                );

            main_stack
        })
}