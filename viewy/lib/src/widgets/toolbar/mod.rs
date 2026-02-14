use crate::core::widget::Widget;
use crate::node::Node;

#[derive(Widget)]
#[widget(style = "./style.scss")]
pub struct Toolbar {
    node: Node,
}

impl Toolbar {
    fn render(&mut self) {}
}
