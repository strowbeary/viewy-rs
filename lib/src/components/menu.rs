use std::borrow::BorrowMut;
use crate::components::*;
use crate::components::badge::{Badge, BadgeSupport};
use crate::components::icons::IconPack;
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

impl DefaultModifiers for MenuSection {}

impl Renderable for MenuSection {
    fn render(mut self) -> Node {
        self.add_class("menu-section");
        self.node.children.push({
            Text::new(&self.label, TextStyle::Overline).render()
        });
        self.node
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
    is_selected: bool,
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
            is_selected: false,
        }
    }
    /// Set menu's icon
    pub fn icon<T>(&mut self, icon: T) -> &mut Self
        where
            T: 'static + IconPack {
        self.icon = Some(Box::new(icon));
        self
    }

    pub fn icon_color(&mut self, color: &str) -> &mut Self {
        self.icon_color = Some(color.to_string());
        self
    }

    pub fn destructive(&mut self) -> &mut Self {
        self.is_destructive = true;
        self
    }

    pub fn selected(&mut self) -> &mut Self {
        self.is_selected = true;
        self
    }

    pub fn action(&mut self, url: &str) -> &mut Self {
        self
            .set_attr("href", url)
            .tag("a")
    }
    pub fn attach_to_file_input(&mut self, input_id: &str) -> &mut Self {
        self
            .set_attr("data-input-file", &format!("file-input-{}", input_id))
    }

    /// Make the `MenuItem` submit specified form
    pub fn attach_to_form(&mut self, form_name: &str) -> &mut Self {
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

impl DefaultModifiers for MenuItem {}


impl BadgeSupport for MenuItem {
    fn add_badge(&mut self, badge: Badge) {
        self.badge = Some(badge);
    }
}

impl BadgeModifiers for MenuItem {}

impl Renderable for MenuItem {
    fn render(mut self) -> Node {
        let destructive_class = match self.is_destructive {
            true => { "menu-item--destructive" }
            false => { "menu-item--normal" }
        };
        self
            .add_class("menu-item")
            .add_class(destructive_class);
        if self.is_selected {
            self.add_class("menu-item--selected");
        }
        if let Some(icon) = self.icon {
            self.node.children.append(&mut vec![{
                let mut icon = Icon::new(icon)
                    .size(16);
                if let Some(icon_color) = &self.icon_color {
                    icon.color(icon_color);
                }
                icon.render()
            }]);
        }

        self.node.children.append(&mut vec![
                Text::new(self.label.as_str(), TextStyle::Label).render()
            ]);


        if let Some(badge) = self.badge {
            self.node.children.push(badge.render());
        }
        self.node
    }
}

#[derive(Debug, Clone)]
pub struct Menu {
    node: Node,
    pub style: MenuStyle,
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

impl DefaultModifiers for Menu {}


impl Renderable for Menu {
    fn render(mut self) -> Node {
        self
            .add_class("menu");
        
        let style = match &self.style {
            MenuStyle::Vertical => {
               "menu--vertical"
            }
            MenuStyle::HorizontalTab => {"menu--horizontal-tab"
            }
            MenuStyle::HorizontalNav => {"menu--horizontal-nav"
            }
        };
        self.add_class(style);
        self.children.into_iter()
            .for_each(|child|
                self.node.children.push(child.render()));


        self.node
    }
}