use crate::node::{Node, NodeContainer};
use crate::{ToHtml, DefaultModifiers};
use crate::renderer::Renderable;
use std::borrow::BorrowMut;
use crate::components::*;


#[derive(Debug, Clone)]
pub enum MenuStyle {
    VerticalNavbar,
    Toolbar,
    RadioGroup,
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    node: Node,
    pub icon: Option<String>,
    pub label: String,
}

impl MenuItem {
    pub fn new(label: &str) -> Self {
        Self {
            node: Default::default(),
            icon: None,
            label: label.to_string(),
        }
    }
    pub fn icon(&mut self, name: &str) -> Self {
        self.icon = Some(name.to_string());
        self.clone()
    }

    pub fn action(&mut self, url: &str) -> Self {
        self
            .set_attr("href", url)
            .tag("a")
    }
}

impl NodeContainer for MenuItem {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<MenuItem> for MenuItem {}

impl Renderable for MenuItem {
    fn render(&self) -> Node {
        let mut menu_item = self
            .clone()
            .add_class("menu-item")
            .add_class("menu-item--normal");
        if let Some(icon) = menu_item.icon.clone() {
            menu_item.node.children.append(&mut vec![
                Icon::new(icon.as_str())
                    .size(16)
                    .stroke_width(5)
                    .render()
            ]);
        }
        menu_item.node.children.append(&mut vec![
            Text::new(self.label.as_str(), TextStyle::Label).render()
        ]);
        menu_item.get_node().clone()
    }
}

#[derive(Debug, Clone)]
pub struct Menu {
    node: Node,
    style: MenuStyle,
    children: Vec<Box<dyn Renderable>>,
}
impl Menu {
    pub fn new(style: MenuStyle) -> Self {
        Self {
            node: Default::default(),
            style,
            children: vec![]
        }
    }
}


impl ChildContainer for Menu {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}
impl Appendable for Menu {}

impl NodeContainer for Menu {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Menu> for Menu {}

impl ToHtml for Menu {}

impl Renderable for Menu {
    fn render(&self) -> Node {
        let mut menu = self
            .clone()
            .add_class("menu");
        self.children.iter()
            .for_each(|child|
                menu.node.children.push(child.render()));
        menu.get_node().clone()
    }
}