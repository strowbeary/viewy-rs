use std::ops::DerefMut;

use viewy::{DefaultModifiers, Renderable, components::*, node::Node, scale};

pub fn login_layout(content: Node) -> Node {
    let mut stack = VStack::new(Alignment::Start);
    stack
        .padding(vec![scale(6), scale(12)])
        .background_color("var(--background-raised)")
        .background_image("https://source.unsplash.com/random/1920x1080?office")
        .width("100%")
        .height("100%")
        .justify_content("center")
        .append_child({
            let mut card = Card::new(CardStyle::Raised);
            card.padding(vec![scale(5), scale(5), scale(6), scale(5)]);
            card.deref_mut().children.push(content);
            card
        });

    stack.render()
}
