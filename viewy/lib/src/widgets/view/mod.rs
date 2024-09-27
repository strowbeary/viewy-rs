use crate::core::modifiers::{Appendable, Classable, Colorable, Dimensionable};
use crate::core::node::Node;
use crate::core::widget::Widget;

#[derive(Widget, Appendable, Colorable, Classable, Dimensionable)]
#[widget(style = "./style.scss")]
pub struct View {
    pub node: Node,
}

impl View {
    pub fn new() -> Self {
        View {
            node: Node::default(),
        }
    }

    pub fn render(&mut self) {}
}
