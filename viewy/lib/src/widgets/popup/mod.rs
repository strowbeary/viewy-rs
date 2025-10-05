use crate::core::modifiers::{Appendable, Attributable};
use crate::core::node::Node;
use crate::modifiers::Classable;
use crate::widgets::view::View;
use crate::Widget;
use uuid::{uuid, Uuid};

#[derive(Widget, Attributable)]
#[widget(style = "./style.scss")]
pub struct Popup {
    node: Node,
    window_content: Node,
    name: String
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
    pub const IDENTIFIER: Uuid = uuid!("45bf3464-7ca3-4548-a647-b0e828b6922a");
    pub fn new(name: &str) -> Self {
        Popup {
            name: name.to_string(),
            node: Node {
                identifier: Self::IDENTIFIER,
                ..Node::default()
            },

            window_content: Node {
                ..Node::default()
            },
        }
    }

    fn render(&mut self) {
        self.node.class_list.insert("popup".to_string());
        self.node.attributes.insert("id".to_string(), format!("popup_{}", self.name));
        self.node.children.push({
            View::new()
                .add_class("popup__window")
                .append_child(View::new().add_class("popup__window__window-bar"))
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
