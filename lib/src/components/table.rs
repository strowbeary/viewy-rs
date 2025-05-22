use std::borrow::BorrowMut;

use crate::{DefaultModifiers, sp};
use crate::components::*;
use crate::node::{Node, NodeContainer};
use crate::Renderable;

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
    pub name: String,
    pub action: Option<String>,
    pub action_target: Option<String>,
    pub download: Option<String>,
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
            name: name.to_string(),
            action: None,
            action_target: None,
            download: None,
        }
    }


    pub fn action(&mut self, url: &str) -> Self {
        self.action = Some(url.to_string());
        self.clone()
    }

    pub fn action_target(&mut self, target: &str) -> Self {
        self.action_target = Some(target.to_string());
        self.clone()
    }

    pub fn download_action(&mut self, url: &str, file_name: &str) -> Self {
        self.action = Some(url.to_string());
        self.download = Some(file_name.to_string());
        self.clone()
    }
}

impl Renderable for Row {
    fn render(&self) -> Node {
        let mut row = self.clone()
            .tag("tr");
        if self.action.is_some() {
            row.add_class("clickable");
        }
        let mut node = row.node;
        row.children.iter()
            .for_each(|child| {
                let mut td = View::new()
                    .tag("td");
                if let Some(url) = &self.action {
                    td.get_children().push({
                        Box::new({
                            let mut link = View::new()
                                .add_class("link-row")
                                .tag("a")
                                .set_attr("href", url)
                                .append_child(child.clone());
                            if let Some(target) = &self.action_target {
                                link.set_attr("target", target);
                            }
                            if let Some(file_name) = &self.download {
                                link.set_attr("download", file_name);
                            }
                            link
                        })
                    })
                } else {
                    td.get_children().push(child.clone());
                }
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
    select_all: bool
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
            select_all: false,
        }
    }

    pub fn selectable(&mut self, is_selectable: bool) -> Self {
        self.selectable = is_selectable;
        self.clone()
    }
    
    pub fn display_select_all_checkbox(&mut self) -> Self {
        self.select_all = true;
        self.clone()
    }

    pub fn alternate_row_color(&mut self, is_alternate: bool) -> Self {
        self.add_class("alternate-row-color");
        self.clone()
    }

    pub fn append_child(&mut self, row: Row) -> Self
    {
        self.rows.push(row);
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

impl Renderable for Table {
    fn render(&self) -> Node {
        let mut table = self.clone()
            .tag("table")
            .add_class("table");
        table.get_node().children.push({
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
        }.render());
        if self.clone().columns.iter().any(|col| col.title.is_some()) {

            table.get_node().children.push({
                View::new()
                    .tag("thead")
                    .append_child({
                        let mut tr = View::new()
                            .tag("tr");
                        if self.selectable {
                            tr.prepend_child({
                                let mut checkbox_th = View::new()
                                    .tag("th");
                                
                                if self.select_all {
                                    checkbox_th.append_child({
                                        Checkbox::new(&self.name, "", CheckboxStyle::Checkbox)
                                            .attach_to_form("dummy")
                                            .add_class("select-all")
                                    });   
                                }
                                checkbox_th
                            });
                        }
                        self.clone().columns.into_iter()
                            .for_each(|col| {
                                if let Some(title) = col.title {
                                    tr.append_child({
                                        Text::new(title.as_str(), TextStyle::Label)
                                            .tag("th")
                                    });
                                } else {
                                    tr.append_child({
                                        View::new().tag("th")
                                    });
                                }
                            });
                        tr
                    })
            }.render());
        }


        table.get_node().children.push({
            let mut tbody = View::new()
                .tag("tbody");
            for mut row in self.clone().rows {
                if self.selectable {
                    row.prepend_child({
                        Checkbox::new(&self.name, &row.name, CheckboxStyle::Checkbox)
                    });
                }
                tbody.append_child(row);
            }
            tbody
        }.render());

        table.node
    }
}