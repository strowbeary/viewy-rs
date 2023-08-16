use crate::components::view::View;
use crate::core::modifiers::{Appendable, Attributable};
use crate::core::node::Node;
use crate::Widget;
#[derive(Widget, Attributable, Appendable)]
pub struct Popup {
    node: Node,
}

impl Popup {
    pub fn new() -> Self {
        Popup {
            node: Node::default()
        }
    }

    fn render(&mut self) {
        self.node.class_list.insert("popup".to_string());
        self.append_child::<_>(View::new());
    }

    pub fn attach_to(&mut self, id: &str) -> &mut Self {
        self.set_attr("data-attach-to", id)
    }
}

