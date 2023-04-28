use std::borrow::BorrowMut;
use crate::components::*;
use crate::components::badge::{Badge, BadgeSupport};
use crate::components::icons::{IconPack, Lucide};
use crate::DefaultModifiers;
use crate::node::{Node, NodeContainer};
use crate::Renderable;

#[derive(Debug, Clone)]
pub enum MenuStyle {
    HorizontalTab,
    HorizontalNav,
    Vertical,
}

//MENU SECTION

#[derive(Debug, Clone)]
pub struct MenuSection {
    node: Node,
    pub label: String,
}

impl MenuSection {
    pub fn new(label: &str) -> Self {
        Self {
            node: Default::default(),
            label: label.to_string(),
        }
    }
}


impl NodeContainer for MenuSection {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<MenuSection> for MenuSection {}

impl Renderable for MenuSection {
    fn render(&self) -> Node {
        let mut menu_section = self.clone()
            .add_class("menu-section");
        menu_section.node.children.push({
            Text::new(self.label.as_str(), TextStyle::Overline).render()
        });
        menu_section.node
    }
}

// MENU ITEM

#[derive(Debug, Clone)]
pub struct MenuItem {
    node: Node,
    pub icon: Option<Box<dyn IconPack>>,
    pub icon_color: Option<String>,
    pub label: String,
    badge: Option<Badge>,
    is_destructive: bool,
}

impl MenuItem {
    pub fn new(label: &str) -> Self {
        Self {
            node: Default::default(),
            icon: None,
            icon_color: None,
            label: label.to_string(),
            badge: None,
            is_destructive: false,
        }
    }
    /// Set menu's icon
    pub fn icon<T>(&mut self, icon: T) -> Self
        where
            T: 'static + IconPack {
        self.icon = Some(Box::new(icon));
        self.clone()
    }

    pub fn icon_color(&mut self, color: &str) -> Self {
        self.icon_color = Some(color.to_string());
        self.clone()
    }

    pub fn destructive(&mut self) -> Self {
        self.is_destructive = true;
        self.clone()
    }

    pub fn action(&mut self, url: &str) -> Self {
        self
            .set_attr("href", url)
            .tag("a")
    }
    pub fn attach_to_file_input(&mut self, input_id: &str) -> Self {
        self
            .set_attr("data-input-file", &format!("file-input-{}", input_id))
    }

    /// Make the `MenuItem` submit specified form
    pub fn attach_to_form(&mut self, form_name: &str) -> Self {
        self
            .set_attr("form", form_name)
            .set_attr("type", "submit")
            .tag("button")
    }
}

impl NodeContainer for MenuItem {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<MenuItem> for MenuItem {}


impl BadgeSupport for MenuItem {
    fn add_badge(&mut self, badge: Badge) {
        self.badge = Some(badge);
    }
}

impl BadgeModifiers for MenuItem {}

impl Renderable for MenuItem {
    fn render(&self) -> Node {
        let mut menu_item = self
            .clone()
            .add_class("menu-item")
            .add_class(match self.is_destructive {
                true => { "menu-item--destructive" }
                false => { "menu-item--normal" }
            });
        if let Some(icon) = menu_item.icon.clone() {
            menu_item.node.children.append(&mut vec![{
                let mut icon = Icon::new(icon)
                    .size(16);
                if let Some(icon_color) = &menu_item.icon_color {
                    icon.color(icon_color);
                }
                icon.render()
            }]);
        }
        if menu_item.node.popover.is_some() {
            menu_item.node.children.append(&mut vec![
                Text::new(self.label.as_str(), TextStyle::Label)
                    .render(),
                Icon::new(Lucide::ChevronDown)
                    .size(16)
                    .render(),
            ]);
        } else {
            menu_item.node.children.append(&mut vec![
                Text::new(self.label.as_str(), TextStyle::Label).render()
            ]);
        }

        if let Some(badge) = &menu_item.badge {
            menu_item.node.children.push(badge.render());
        }
        menu_item.node
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
            children: vec![],
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


impl Renderable for Menu {
    fn render(&self) -> Node {
        let mut menu = self
            .clone()
            .add_class("menu");
        match self.style {
            MenuStyle::Vertical => {
                menu = menu.add_class("menu--vertical")
            }
            MenuStyle::HorizontalTab => {
                menu = menu.add_class("menu--horizontal-tab")
            }
            MenuStyle::HorizontalNav => {
                menu = menu.add_class("menu--horizontal-nav")
            }
        }
        self.children.iter()
            .for_each(|child|
                menu.node.children.push(child.render()));


        menu.get_node().clone()
    }
}