use crate::{Renderable};
use crate::node::{Node, NodeContainer};
use crate::components::*;
use std::borrow::BorrowMut;
use crate::{DefaultModifiers, sp};


#[derive(Debug, Clone)]
pub struct Column {
    pub node: Node,
    pub title: Option<String>,
}

impl NodeContainer for Column {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Column> for Column {}

impl Column {
    pub fn new(title: Option<&str>) -> Self {
        Self {
            node: Default::default(),
            title: title.map(|title| title.to_string()),
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
pub struct Row {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    pub name: String
}

impl NodeContainer for Row {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Row> for Row {}

impl ChildContainer for Row {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl Appendable for Row {}

impl Row {
    pub fn new(name: &str) -> Self {
        Self {
            children: vec![],
            node: Default::default(),
            name: name.to_string()
        }
    }
}

impl Renderable for Row {
    fn render(&self) -> Node {
        let mut row = self.clone()
            .tag("tr");
        let mut node = row.node;
        row.children.iter()
            .for_each(|child| {
                let mut td = View::new()
                    .tag("td");
                td.get_children().push(child.clone());
                node.children.push(td.render())
            });
        node
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    name: String,
    children: Vec<Box<dyn Renderable>>,
    columns: Vec<Column>,
    rows: Vec<Row>,
    node: Node,
    selectable: bool,
}



impl Table {
    pub fn new(name: &str, columns: Vec<Column>) -> Self {
        Self {
            name: name.to_string(),
            children: vec![],
            columns,
            rows: vec![],
            node: Default::default(),
            selectable: false,
        }
    }

    pub fn append_row(&mut self, row: Row) -> Self {
        self.rows.push(row);
        self.clone()
    }

    pub fn selectable(&mut self, is_selectable: bool) -> Self {
        self.selectable = is_selectable;
        self.clone()
    }
}

impl NodeContainer for Table {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Table> for Table {}

impl ChildContainer for Table {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl Appendable for Table {}

impl Renderable for Table {
    fn render(&self) -> Node {
        let mut table = self
            .clone()
            .tag("table");
        table.append_child({
            let mut colgroup = View::new()
                .tag("colgroup");
            if self.selectable {
                colgroup.append_child({
                    Column::new(None)
                        .width(&sp(30))
                });
            }
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
                let mut tr = View::new()
                    .tag("tr");
                if self.selectable {
                    tr.prepend_child({
                        View::new()
                            .tag("th")
                    });
                }
                self.clone().columns.into_iter()
                    .for_each(|col| {
                        if let Some(title) = col.title {
                            tr.append_child({
                                Text::new(title.as_str(), TextStyle::Headline)
                                    .tag("th")
                            });
                        } else {
                            tr.append_child({
                                View::new().tag("th")
                            });
                        }
                    });
                tr
            });
            thead
        });

        table.append_child({
            let mut tbody = View::new()
                .tag("tbody");
            self.clone().rows.into_iter()
                .for_each(|mut row| {
                    if self.selectable {
                        row.prepend_child({
                            Checkbox::new(&table.name, &row.name)
                        });
                    }
                    tbody.append_child(row);
                });
            tbody
        });

        let mut view = table.node;
        table.children.iter()
            .for_each(|child|
                view.children.push(child.render()));
        view
    }
}