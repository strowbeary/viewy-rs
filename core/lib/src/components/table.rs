use crate::renderer::{Renderable, ToHtml};
use crate::node::{Node, NodeContainer};
use crate::components::{Alignment, Text, TextStyle, View};
use std::borrow::BorrowMut;
use crate::{DefaultModifiers, sp};
use crate::component::{Appendable, ChildContainer};

#[derive(Debug, Clone)]
pub struct Column {
    pub node: Node,
    pub title: String
}

impl NodeContainer for Column {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Column> for Column {}

impl ToHtml for Column {}

impl Column {
    pub fn new(title: &str) -> Self {
        Self {
            node: Default::default(),
            title: title.to_string()
        }
    }

    pub fn span(&mut self, col_nbs: i32) -> Self {
        self.set_attr("span", col_nbs.to_string().as_str());
        self.clone()
    }
}

impl Renderable for Column {
    fn render(&self) -> Node {
        self.clone()
            .tag("col")
            .node
    }

}

#[derive(Debug, Clone)]
pub struct Table {
    children: Vec<Box<dyn Renderable>>,
    columns: Vec<Column>,
    node: Node,
}

impl NodeContainer for Table {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Table> for Table {}

impl ToHtml for Table {}

impl ChildContainer for Table {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl Appendable<Table> for Table {}

impl Renderable for Table {
    fn render(&self) -> Node {
        let mut table = self
            .clone()
            .tag("table");
        table.append_child({
            let mut colgroup = View::new()
                .tag("colgroup");
            self.clone().columns.into_iter()
                .for_each(|col| {
                    colgroup.append_child(col);
                });
            colgroup
        });
        table.append_child({
            let mut thead = View::new()
                .tag("thead");
            thead.append_child({
                let mut header = View::new()
                    .tag("tr");
                self.clone().columns.into_iter()
                    .for_each(|col| {
                        header.append_child({
                            Text::new(col.title.as_str(), TextStyle::Headline)
                                .tag("th")
                        });
                    });
                header
            });
            thead
        });


        let mut view = table.node;
        table.children.iter()
            .for_each(|child|
                view.children.push(child.render()));
        view

    }
}

impl Table {
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            children: vec![],
            columns,
            node: Default::default(),
        }
    }
}