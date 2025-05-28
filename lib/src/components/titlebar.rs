use std::borrow::BorrowMut;

use crate::DefaultModifiers;
use crate::Renderable;
use crate::components::{Alignment, Appendable, Text, TextStyle, VStack};
use crate::node::Node;

#[derive(Debug, Clone)]
pub struct TitleBar {
    node: Node,
    pub title: String,
    pub subtitle: Option<String>,
    pub is_sticky: bool,
    pub left_item: Option<Node>,
    pub right_item: Option<Node>,
    pub bottom_item: Option<Node>,
}
impl std::ops::Deref for TitleBar {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for TitleBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}
impl DefaultModifiers for TitleBar {}

impl TitleBar {
    pub fn new(title: &str) -> Self {
        TitleBar {
            node: Default::default(),
            title: title.to_string(),
            subtitle: None,
            is_sticky: true,
            left_item: None,
            right_item: None,
            bottom_item: None,
        }
    }

    pub fn subtitle(&mut self, subtitle: &str) -> &mut Self {
        self.subtitle = Some(subtitle.to_string());
        self
    }
    fn grid_areas(&mut self, schema: &str) -> &mut Self {
        self.node
            .node_style
            .push(("grid-template-areas".to_string(), schema.to_string()));
        self
    }

    pub fn sticky(&mut self, is_sticky: bool) -> &mut Self {
        self.is_sticky = is_sticky;
        self
    }

    pub fn left_item<T>(&mut self, item: T) -> &mut Self
    where
        T: Renderable,
    {
        self.left_item = Some(item.render());
        self
    }
    pub fn right_item<T>(&mut self, item: T) -> &mut Self
    where
        T: Renderable,
    {
        self.right_item = Some(item.render());
        self
    }
    pub fn bottom_item<T>(&mut self, item: T) -> &mut Self
    where
        T: Renderable,
    {
        self.bottom_item = Some(item.render());
        self
    }
}

impl Renderable for TitleBar {
    fn render(mut self) -> Node {
        let mut areas = String::new();
        if self.left_item.is_some() {
            areas.push_str("'left_item . right_item' 'title title title'");
        } else {
            areas.push_str("'title title right_item'");
        }
        if self.bottom_item.is_some() {
            areas.push_str("'bottom_item bottom_item bottom_item'");
        }
        self.add_class("titlebar").grid_areas(areas.as_str());

        if self.is_sticky {
            self.sticky_to_top(0);
        }

        self.node.children.push({
            if let Some(subtitle) = &self.subtitle {
                let mut stack = VStack::new(Alignment::Stretch);
                stack
                    .grid_area("title")
                    .append_child({ Text::new(self.title.as_str(), TextStyle::LargeTitle) })
                    .append_child({ Text::new(subtitle, TextStyle::H3) });
                stack.render()
            } else {
                let mut text = Text::new(self.title.as_str(), TextStyle::LargeTitle);
                text.grid_area("title");
                text.render()
            }
        });
        if let Some(mut left_item) = self.left_item {
            left_item
                .node_style
                .push(("grid-area".to_string(), "left_item".to_string()));
            self.node.children.push(left_item);
        }
        if let Some(mut right_item) = self.right_item {
            right_item
                .node_style
                .push(("grid-area".to_string(), "right_item".to_string()));
            self.node.children.push(right_item);
        }
        if let Some(mut bottom_item) = self.bottom_item {
            bottom_item
                .node_style
                .push(("grid-area".to_string(), "bottom_item".to_string()));
            self.node.children.push(bottom_item);
        }
        self.node
    }
}
