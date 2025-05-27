use std::borrow::BorrowMut;
use uuid::Uuid;

use crate::{DefaultModifiers, Renderable, scale};
use crate::components::{Alignment, Appendable, ChildContainer, HStack, Icon, Text, TextStyle, View};
use crate::components::icons::IconPack;
use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone)]
pub struct TabView {
    node: Node,
    children: Vec<TabViewItem>,
}

impl TabView {
    pub fn new() -> Self {
        Self {
            node: Default::default(),
            children: vec![],
        }
    }

    pub fn append_child(&mut self, child: TabViewItem) -> &mut Self {
        self.children.push(child);
        self
    }
}

impl DefaultModifiers for TabView {}

impl NodeContainer for TabView {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl Renderable for TabView {
    fn render(mut self) -> Node {
        self.add_class("tab-view");

        self.node.children.push({
            let mut tab_bar = HStack::new(Alignment::Stretch);
            tab_bar.add_class("tab-view__tab-container");
            self.children.iter()
                .for_each(|child| {
                    tab_bar.append_child({
                        let mut tab = HStack::new(Alignment::Center);
                        tab.gap(vec![scale(3)])
                            .set_attr("data-tabId", &child.id.to_string())
                            .add_class("tab-view__tab-container__tab")
                            .append_child({
                                Text::new(&child.title, TextStyle::Label)
                            });
                        if let Some(icon) = child.icon.clone() {
                            tab.prepend_child({
                                Icon::new(icon)
                                    .size(18)
                                    .stroke_width(2)
                            });
                        }
                        if child.open {
                            tab.set_attr("data-is-open", "data-is-open");
                            tab.add_class("tab-view__tab-container__tab--active");
                        }
                        tab
                    });
                });
            tab_bar
        }.render());

        self.node.children.push({
            let mut tabs_contents = View::new();
            tabs_contents.add_class("tab-view__content-container");
            self.children.into_iter()
                .for_each(|child| {
                    tabs_contents.append_child({
                        let mut tab_content = View::new();
                        tab_content
                            .set_attr("data-tabId", &child.id.to_string())
                            .add_class("tab-view__content-container__content");
                        child.children.into_iter().for_each(|child| {
                            tab_content.append_child(child);
                        });
                        tab_content
                    });
                });
            tabs_contents
        }.render());


        self.node
    }
}

#[derive(Debug, Clone)]
pub struct TabViewItem {
    node: Node,
    pub id: Uuid,
    pub title: String,
    pub icon: Option<Box<dyn IconPack>>,
    pub open: bool,
    children: Vec<Box<dyn Renderable>>,
}

impl TabViewItem {
    pub fn new(title: &str) -> TabViewItem {
        TabViewItem {
            node: Default::default(),
            id: Uuid::new_v4(),
            title: title.to_string(),
            icon: None,
            open: false,
            children: vec![],
        }
    }

    /// If set to true, this tab will be opened by default
    pub fn open(&mut self, is_open: bool) -> &mut Self {
        self.open = is_open;
        self
    }

    /// Define the icon displayed on the left of the tab
    pub fn icon<T>(&mut self, icon: T) -> &mut Self
        where
            T: 'static + IconPack {
        self.icon = Some(Box::new(icon));
        self
    }
}

impl DefaultModifiers for TabViewItem {}

impl Appendable for TabViewItem {}

impl ChildContainer for TabViewItem {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl NodeContainer for TabViewItem {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl Renderable for TabViewItem {
    fn render(mut self) -> Node {
        self.add_class("tab-view-item");

        self.node
    }
}