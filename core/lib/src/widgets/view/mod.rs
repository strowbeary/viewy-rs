use crate::core::node::{Node};
use crate::core::widget::Widget;
use crate::core::modifiers::{Appendable, Colorable, Dimensionable};

#[derive(Widget, Appendable, Colorable)]
pub struct View {
    node: Node
}

impl View {
    pub fn new() -> Self {
        View {
            node: Node::default()
        }
    }

    pub fn render(&mut self) {

    }
}


impl Dimensionable for View {}