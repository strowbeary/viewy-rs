use crate::Widget;
use crate::modifiers::*;
use crate::node::Node;
use crate::widgets::stack::{Alignment, Stack};

#[derive(Widget, Appendable, Colorable, Classable, Cardifiable)]
#[widget(style = "./style.scss")]
pub struct VStack {
    node: Node,
}

impl Stack for VStack {
    fn new(alignment: Alignment) -> Self {
        let mut stack = VStack {
            node: Node::default(),
        };
        stack
            .add_class("stack")
            .add_class("stack--vertical")
            .add_class(
                format!("stack--align-{:?}", alignment)
                    .to_lowercase()
                    .as_str(),
            );
        stack
    }
}

impl Paddingable for VStack {}
