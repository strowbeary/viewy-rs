use std::borrow::BorrowMut;
use uuid::Uuid;

use crate::{DefaultModifiers, Renderable, scale, sp};
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

    pub fn append_child(&mut self, child: TabViewItem) -> Self {
        self.children.push(child);
        self.clone()
    }
}

impl DefaultModifiers<TabView> for TabView {}

impl NodeContainer for TabView {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl Renderable for TabView {
    fn render(&self) -> Node {
        let mut tab_view = self
            .clone()
            .add_class("tab-view");

        tab_view.node.children.push({
            let mut tab_bar = HStack::new(Alignment::Stretch)
                .add_class("tab-view__tab-container");
            self.children.iter()
                .for_each(|child| {
                    tab_bar.append_child({
                        let mut tab = HStack::new(Alignment::Center)
                            .gap(vec![scale(3)])
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

        tab_view.node.children.push({
            let mut tabs_contents = View::new()
                .add_class("tab-view__content-container");
            self.children.iter()
                .for_each(|child| {
                    tabs_contents.append_child({
                        let mut tab_content = View::new()
                            .add_class("")
                            .set_attr("data-tabId", &child.id.to_string())
                            .add_class("tab-view__content-container__content");
                        child.children.clone().into_iter().for_each(|child| {
                            tab_content.append_child(child);
                        });
                        tab_content
                    });
                });
            tabs_contents
        }.render());


        tab_view.get_node().clone()
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
    pub fn open(&mut self, is_open: bool) -> Self {
        self.open = is_open;
        self.clone()
    }

    /// Define the icon displayed on the left of the tab
    pub fn icon<T>(&mut self, icon: T) -> Self
        where
            T: 'static + IconPack {
        self.icon = Some(Box::new(icon));
        self.clone()
    }
}

impl DefaultModifiers<TabViewItem> for TabViewItem {}

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
    fn render(&self) -> Node {
        let mut tab_view_item = self
            .clone()
            .add_class("tab-view-item");

        tab_view_item.get_node().clone()
    }
}