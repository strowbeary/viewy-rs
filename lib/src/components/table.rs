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

impl DefaultModifiers for Column {}

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
    fn render(mut self) -> Node {
        self.tag("col");
        self.node
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

impl DefaultModifiers for Row {}

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
    fn render(mut self) -> Node {
        self.tag("tr");
        if self.action.is_some() {
            self.add_class("clickable");
        }
        self.children.into_iter()
            .for_each(|child| {
                let mut td = View::new();
                td.tag("td");

                if let Some(url) = &self.action {
                    td.append_child({
                        let mut link = View::new();
                        link.add_class("link-row")
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
                    });
                } else {
                    td.append_child(child);
                }
                self.node.children.push(td.render())
            });
        self.node
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

    pub fn selectable(&mut self, is_selectable: bool) -> &mut Self {
        self.selectable = is_selectable;
        self
    }
    
    pub fn display_select_all_checkbox(&mut self) -> &mut Self {
        self.select_all = true;
        self
    }

    pub fn alternate_row_color(&mut self) -> &mut Self {
        self.add_class("alternate-row-color")
    }

    pub fn append_child(&mut self, row: Row) -> &mut Self
    {
        self.rows.push(row);
        self
    }
}

impl NodeContainer for Table {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers for Table {}

impl ChildContainer for Table {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl Renderable for Table {
    fn render(mut self) -> Node {
        let table_has_header = self.columns.iter().any(|col| col.title.is_some());
        self
            .tag("table")
            .add_class("table");

        if table_has_header {

            self.node.children.push({
                let mut thead = View::new();
                thead.tag("thead")
                    .append_child({
                        let mut tr = View::new();
                        tr .tag("tr");
                        if self.selectable {
                            tr.prepend_child({
                                let mut checkbox_th = View::new();
                                checkbox_th.tag("th");
                                
                                if self.select_all {
                                    checkbox_th.append_child({
                                        let mut checkbox = Checkbox::new(&self.name, "", CheckboxStyle::Checkbox);
                                        checkbox.attach_to_form("dummy")
                                            .add_class("select-all");
                                        checkbox
                                    });   
                                }
                                checkbox_th
                            });
                        }
                        self.columns.iter()
                            .for_each(|col| {
                                if let Some(title) = &col.title {
                                    tr.append_child({
                                        let mut th = Text::new(title, TextStyle::Label);
                                        th.tag("th");
                                        th
                                    });
                                } else {
                                    tr.append_child({
                                        let mut th = View::new();
                                        th.tag("th");
                                        th
                                    });
                                }
                            });
                        tr
                    });
                thead
            }.render());
        }
        self.node.children.insert(0, {
            let mut colgroup = View::new();
            colgroup.tag("colgroup");
            if self.selectable {
                colgroup.append_child({
                    let mut col = Column::new(None);
                    col.width(&sp(30));
                    col
                });
            }
            self.columns.into_iter()
                .for_each(|col| {
                    colgroup.append_child(col);
                });
            colgroup
        }.render());

        self.node.children.push({
            let mut tbody = View::new();
            tbody.tag("tbody");
            for mut row in self.rows {
                if self.selectable {
                    row.prepend_child({
                        Checkbox::new(&self.name, &row.name, CheckboxStyle::Checkbox)
                    });
                }
                tbody.append_child(row);
            }
            tbody
        }.render());

        self.node
    }
}