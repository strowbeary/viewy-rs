use viewy::components::*;
use viewy::{DefaultModifiers, Renderable};

pub fn showcase_section<C>(title: &str, content: C) -> Card
    where C: 'static + Renderable {
    Card::new(CardStyle::Raised)
        .padding(vec![30])
        .append_child({
           HStack::new(Alignment::Center)
               .margin_bottom(25)
               .justify_content("space-between")
               .append_child({
                   Text::new(title, TextStyle::H1)
               })
               .append_child({
                   Button::new("See documentation", ButtonStyle::Link)
                       .icon("book-open")
               })
        })
        .append_child(Divider::new())
        .append_child(content)
}