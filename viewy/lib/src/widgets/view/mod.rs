use crate::modifiers::{Appendable, Classable, Colorable, Dimensionable, Cardifiable};
use crate::core::node::Node;
use crate::core::widget::Widget;

#[derive(Widget, Appendable, Colorable, Classable, Dimensionable, Cardifiable)]
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
