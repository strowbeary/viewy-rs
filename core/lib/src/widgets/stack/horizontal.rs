
use crate::modifiers::*;
use crate::{ Widget};
use crate::prelude::Node;
use crate::widgets::stack::{Alignment, Stack};

#[derive(Widget, Appendable, Colorable, Classable)]
#[widget(style = "./style.scss")]
pub struct HStack {
    node: Node,
}

impl Stack for HStack {
    fn new(alignment: Alignment) -> Self {
        let mut stack = HStack {
            node: Node::default()
        };
        stack.add_class("stack")
            .add_class("stack--horizontal")
            .add_class(format!("stack--align-{:?}", alignment).to_lowercase().as_str());
        stack
    }
}

