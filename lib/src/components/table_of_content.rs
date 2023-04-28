use crate::node::{Node, NodeContainer};
use crate::{DefaultModifiers, Renderable};
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub struct TableOfContentsItem {
    node: Node,
    children: Vec<TableOfContentsItem>,
    pub label: String,
}

impl TableOfContentsItem {
    pub fn new(label: &str) -> Self {
        Self {
            node: Default::default(),
            children: vec![],
            label: label.to_string(),
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
    children: Vec<TableOfContentsItem>,
}

impl TableOfContents {
    pub fn new() -> Self {
        Self {
            node: Default::default(),
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
            .add_class("table-of-contents");

        toc.get_node().clone()
    }
}