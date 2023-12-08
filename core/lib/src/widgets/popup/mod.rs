use uuid::uuid;
use crate::widgets::view::View;
use crate::core::modifiers::{Appendable, Attributable};
use crate::core::node::Node;
use crate::Widget;
#[derive(Widget, Attributable, Appendable)]
#[widget(style = "./style.scss", script = "./script.js")]
pub struct Popup {
    node: Node,
}


impl Popup {
    pub fn new() -> Self {
        Popup {
            node: Node {
                identifier: uuid!("45bf3464-7ca3-4548-a647-b0e828b6922a"),
                ..Node::default()
            }
        }
    }

    fn render(&mut self) {
        self.node.class_list.insert("popup".to_string());
        self.append_child(View::new());
    }

    pub fn attach_to(&mut self, id: &str) -> &mut Self {
        self.set_attr("data-attach-to", id)
    }
}

impl AsMut<Popup> for &mut Popup {
    fn as_mut(&mut self) -> &mut Popup {
        self
    }
}