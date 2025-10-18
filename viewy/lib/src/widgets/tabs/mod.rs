use crate::Widget;
use crate::bindings::uri::Uri;
use crate::modifiers::Appendable;
use crate::node::{Node, NodeType};
use crate::prelude::Classable;
use crate::prelude::{Attributable, View};

#[derive(Widget, Classable, Attributable)]
#[widget(style = "./style.scss")]
pub struct TabContainer {
    node: Node,
    keep_content_mounted: bool,
    tabs: Vec<Tab>,
}

impl TabContainer {
    pub fn new() -> Self {
        Self {
            node: Default::default(),
            keep_content_mounted: false,
            tabs: vec![],
        }
    }

    /// Instead of reloading tab content each time it's selected, viewy keep the content already loaded and displays it again.
    pub fn keep_content_mounted(&mut self) -> &mut Self {
        self.keep_content_mounted = true;
        self
    }

    pub fn add_tab(&mut self, tab: Tab) -> &mut Self {
        self.tabs.push(tab);
        self
    }
    fn render(&mut self) {
        self.add_class("tab-container");
        let mut tab_button_container = View::new();
        tab_button_container.add_class("tab-container__button-container");
        for tab in &self.tabs {
            tab_button_container.append_child({
                let mut tab_button = View::new();
                tab_button.add_class("tab-container__button-container__tab");
                tab_button.node.node_type = NodeType::Normal("button");
                tab_button.text = Some(tab.label.clone());
                tab_button
                    .attributes
                    .insert("data-v-url".to_string(), tab.content_url.to_string());
                tab_button
            });
        }

        self.children.push(tab_button_container.into());

        let mut tab_content_panel = View::new();
        tab_content_panel.add_class("tab-container__tab-content");
        self.children.push(tab_content_panel.into());
    }
}

pub struct Tab {
    label: String,
    content_url: Uri,
}

impl Tab {
    pub fn new(label: &str, content_url: Uri) -> Self {
        Self {
            label: label.to_string(),
            content_url,
        }
    }
}
