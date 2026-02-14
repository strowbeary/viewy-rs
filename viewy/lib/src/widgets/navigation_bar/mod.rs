use crate::core::widget::Widget;
use crate::node::Node;

#[derive(Widget)]
#[widget(style = "./style.scss")]
pub struct NavigationBar {
    node: Node,
}

impl NavigationBar {
    fn render(&mut self) {}
}
