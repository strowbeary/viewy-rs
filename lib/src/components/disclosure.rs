use crate::components::{Alignment, Appendable, HStack, Icon, icons::Lucide};
use std::borrow::BorrowMut;
use std::ops::DerefMut;

use crate::DefaultModifiers;
use crate::node::Node;
use crate::{Renderable, scale};

/// Draw a horizontal line to separate content.
#[derive(Debug, Clone)]
pub struct Disclosure {
    node: Node,
    pub opener_item: Option<Node>,

    open: bool,
}

impl DefaultModifiers for Disclosure {}

impl Appendable for Disclosure {}

impl Disclosure {
    pub fn new() -> Self {
        Disclosure {
            node: Node::default(),
            opener_item: None,
            open: false,
        }
    }
    pub fn opener_item<T>(&mut self, item: T) -> &mut Self
    where
        T: Renderable,
    {
        self.opener_item = Some(item.render());
        self
    }

    pub fn open(&mut self, is_open: bool) -> &mut Self {
        self.open = is_open;
        self
    }
}
impl std::ops::Deref for Disclosure {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for Disclosure {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl Renderable for Disclosure {
    fn render(mut self) -> Node {
        self.tag("details").add_class("disclosure");
        if self.open {
            self.set_attr("open", "open");
        }

        if let Some(opener_item) = self.opener_item {
            self.node.children.insert(0, {
                let mut stack = HStack::new(Alignment::Center);
                stack
                    .gap(vec![scale(3)])
                    .tag("summary")
                    .add_class("clickable");
                stack.deref_mut().children.push(opener_item);
                stack.append_child({
                    let mut icon = Icon::new(Lucide::ChevronDown);
                    icon.stroke_width(2);
                    icon
                });
                stack.render()
            });
        }

        self.node
    }
}
