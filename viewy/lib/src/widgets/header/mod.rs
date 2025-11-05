use crate::core::widget::Widget;
use crate::prelude::{Classable, Attributable};
use crate::node::Node;

#[derive(Widget, Classable, Attributable)]
#[widget(style = "./style.scss")]
pub struct Header {
    node: Node,
    main_left_item: Node,
    main_right_item: Node,
}

impl Header {
    fn render(&mut self) {}
}
