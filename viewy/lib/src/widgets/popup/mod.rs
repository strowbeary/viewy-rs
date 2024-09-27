use crate::core::modifiers::{Appendable, Attributable};
use crate::core::node::Node;
use crate::modifiers::Classable;
use crate::node::NodeType;
use crate::widgets::view::View;
use crate::Widget;
use uuid::uuid;

use super::button::Button;
#[derive(Widget, Attributable)]
#[widget(style = "./style.scss", script = "./script.js")]
pub struct Popup {
    node: Node,
    window_content: Node,
}

impl Appendable for Popup {
    fn append_child<C>(&mut self, child: C) -> &mut Self
    where
        C: Into<Node>,
    {
        let node: &mut Node = &mut self.window_content;
        node.children.push(child.into());
        self
    }

    fn set_children(&mut self, children: Vec<Node>) -> &mut Self {
        let node: &mut Node = &mut self.window_content;
        node.children = children;
        self
    }
}

impl Popup {
    pub fn new() -> Self {
        Popup {
            node: Node {
                identifier: uuid!("45bf3464-7ca3-4548-a647-b0e828b6922a"),
                ..Node::default()
            },

            window_content: Node {
                identifier: uuid!("45bf3464-7ca3-4548-a647-b0e828b6922a"),
                ..Node::default()
            },
        }
    }

    fn render(&mut self) {
        self.node.class_list.insert("popup".to_string());
        self.node.children.push({
            View::new()
                .add_class("popup__window")
                .append_child({ View::new().add_class("popup__window__window-bar") })
                .append_child({
                    let mut content = self.window_content.clone();
                    content
                        .class_list
                        .insert("popup__window__window-content".to_string());
                    content
                })
                .into()
        });
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
