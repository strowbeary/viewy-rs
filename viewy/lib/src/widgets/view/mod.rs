use crate::core::node::Node;
use crate::core::widget::Widget;
use crate::modifiers::{Appendable, Cardifiable, Classable, Colorable, Dimensionable};

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
