use std::borrow::BorrowMut;

use crate::{DefaultModifiers, Renderable};
use crate::components::{Text, TextStyle};
use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone)]
pub enum TableOfContentItemType {
    H1,
    H2,
    H3,
}

#[derive(Debug, Clone)]
pub struct TableOfContentsItem {
    children: Vec<TableOfContentsItem>,
    pub label: String,
    pub referrer_id: String,
    pub item_type: TableOfContentItemType,
}

impl TableOfContentsItem {
    pub fn new(label: &str, referrer_id: &str, item_type: TableOfContentItemType) -> Self {
        Self {
            children: vec![],
            label: label.to_string(),
            referrer_id: referrer_id.to_string(),
            item_type,
        }
    }
    pub fn append_child(&mut self, item: TableOfContentsItem) -> Self {
        self.children.push(item);
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub struct TableOfContents {
    node: Node,
    root_id: String,
    children: Vec<TableOfContentsItem>,
}

impl TableOfContents {
    pub fn new(root_id: &str) -> Self {
        Self {
            node: Default::default(),
            root_id: root_id.to_string(),
            children: vec![],
        }
    }
    pub fn append_child(&mut self, item: TableOfContentsItem) -> Self {
        self.children.push(item);
        self.clone()
    }
}

impl NodeContainer for TableOfContents {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers for TableOfContents {}

impl Renderable for TableOfContents {
    fn render(mut self) -> Node {
        self.set_attr("data-root", &self.root_id.to_string())
            .add_class("table-of-contents");

        for child in self.children {
            self.node.children.push({
                let mut text = match child.item_type {
                    TableOfContentItemType::H1 => {
                        Text::new(&child.label, TextStyle::H2)
                    }
                    TableOfContentItemType::H2 => {
                        Text::new(&child.label, TextStyle::Subtitle2)
                    }
                    TableOfContentItemType::H3 => {
                        Text::new(&child.label, TextStyle::Subtitle3)
                    }
                };
                text.add_class("table-of-contents__item")
                    .tag("a")
                    .set_attr("href", &child.referrer_id);
                text.render()
            });
        }

        self.node
    }
}