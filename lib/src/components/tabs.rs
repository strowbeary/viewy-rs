use std::borrow::BorrowMut;
use uuid::Uuid;

use crate::components::icons::IconPack;
use crate::components::{Alignment, Appendable, HStack, Icon, Text, TextStyle, View};
use crate::node::Node;
use crate::{DefaultModifiers, Renderable, scale};

#[derive(Debug, Clone)]
pub struct TabView {
    node: Node,
    tabs: Vec<TabViewItem>,
}

impl TabView {
    pub fn new() -> Self {
        Self {
            node: Default::default(),
            tabs: vec![],
        }
    }

    pub fn append_child(&mut self, child: TabViewItem) -> &mut Self {
        self.tabs.push(child);
        self
    }
}
impl std::ops::Deref for TabView {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for TabView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl DefaultModifiers for TabView {}

impl Renderable for TabView {
    fn render(mut self) -> Node {
        self.add_class("tab-view");
        let mut tab_bar = HStack::new(Alignment::Stretch);
        tab_bar.add_class("tab-view__tab-container");

        let mut tabs_contents = View::new();
        tabs_contents.add_class("tab-view__content-container");

        for tabs in self.tabs {
            tab_bar.append_child({
                let mut tab = HStack::new(Alignment::Center);
                tab.gap(vec![scale(3)])
                    .set_attr("data-tabId", &tabs.id.to_string())
                    .add_class("tab-view__tab-container__tab")
                    .append_child(Text::new(&tabs.title, TextStyle::Label));
                if let Some(icon) = tabs.icon {
                    tab.prepend_child(Icon::new(icon).size(18).stroke_width(2));
                }
                if tabs.open {
                    tab.set_attr("data-is-open", "data-is-open")
                        .add_class("tab-view__tab-container__tab--active");
                }
                tab
            });
            tabs_contents.append_child({
                let mut tab_content = View::new();
                tab_content
                    .set_attr("data-tabId", &tabs.id.to_string())
                    .add_class("tab-view__content-container__content");
                tab_content.node.children = tabs.node.children;
                tab_content
            });
        }
        self.node.children.push(tab_bar.render());
        self.node.children.push(tabs_contents.render());

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
}

impl TabViewItem {
    pub fn new(title: &str) -> TabViewItem {
        TabViewItem {
            node: Default::default(),
            id: Uuid::new_v4(),
            title: title.to_string(),
            icon: None,
            open: false,
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
        T: 'static + IconPack,
    {
        self.icon = Some(Box::new(icon));
        self
    }
}
impl std::ops::Deref for TabViewItem {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for TabViewItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl DefaultModifiers for TabViewItem {}

impl Appendable for TabViewItem {}

impl Renderable for TabViewItem {
    fn render(mut self) -> Node {
        self.add_class("tab-view-item");

        self.node
    }
}
