use crate::node::{Node, NodeContainer};
use crate::{DefaultModifiers, Renderable};
use std::borrow::BorrowMut;
use crate::components::{Text, TextStyle};

#[derive(Debug, Clone)]
pub struct TableOfContentsItem {
    children: Vec<TableOfContentsItem>,
    pub label: String,
    pub referrer_id: String,
}

impl TableOfContentsItem {
    pub fn new(label: &str, referrer_id: &str) -> Self {
        Self {
            children: vec![],
            label: label.to_string(),
            referrer_id: referrer_id.to_string(),
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

impl DefaultModifiers<TableOfContents> for TableOfContents {}

impl Renderable for TableOfContents {
    fn render(&self) -> Node {
        let mut toc = self
            .clone()
            .set_attr("data-root", &self.root_id)
            .add_class("table-of-contents");

        for child in &toc.children {
            toc.node.children.push({
                Text::new(&child.label, TextStyle::Label)
                    .add_class("table-of-contents__item")
                    .tag("a")
                    .set_attr("href", &child.referrer_id)
                    .render()
            });
        }

        toc.get_node().clone()
    }
}