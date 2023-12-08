use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::core::config::Config;
use crate::core::page::html_page::get_full_html_page;
use crate::core::theme::Theme;
use crate::node::{Node, NodeType};

mod html_page;

/// `RenderMode` enum is used to determine how to render a `Page`.
pub enum RenderMode {
    /// This mode will result in a complete HTML page, with the page content wrapped within the page's `Layout`.
    Complete,
    /// This mode will just render the page content without any layout.
    ContentOnly,
    /// Not used for the moment
    LayoutOnly,
}


/// `Layout` is a type alias for a function that takes a `Node` and returns a `Node`.
/// This function will be used to transform the content of a Page.
///
/// # Example
/// A layout function might be used to wrap page content within a common site layout.
///
/// ```rust
/// use viewy::prelude::*;
/// fn layout(content: Node) -> Node {
///    // Components that compose the layout
/// }
///
/// let page = Page::with_title("Page Title")
///     .with_content(View::new())
///     .with_layout(&layout);
/// ```
pub type Layout<'a> = &'a dyn Fn(Node) -> Node;

pub struct Page<'a> {
    content: Node,
    title: String,
    config: Config,
    theme: Theme,
    layout: Layout<'a>,
}

const fn default_layout(content: Node) -> Node {
    content
}

impl<'a> Page<'a> {
    pub fn with_title(title: &str) -> Self {
        Self {
            content: Default::default(),
            title: title.to_string(),
            config: Config::default(),
            theme: Theme::Auto,
            layout: &default_layout,
        }
    }
    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn with_content<C>(mut self, content: C) -> Self
        where C: Into<Node> {
        self.content = content.into();
        self
    }

    pub fn with_layout(mut self, layout: Layout<'a>) -> Self {
        self.layout = layout.into();
        self
    }

    pub fn compile(self, render_mode: RenderMode) -> String {
        let theme_variant = self.theme.as_str();

        match render_mode {
            RenderMode::Complete => {
                get_full_html_page(&self.config, self.title, {
                    let mut content = (self.layout)(self.content);
                    let root_nodes=  content.get_root_nodes();

                    let mut content_str: String = content.into();
                    content_str.push_str(&root_nodes.into_iter()
                        .collect::<Vec<String>>()
                        .join(""));
                    content_str
                }, theme_variant.to_string(), false)
            }
            RenderMode::ContentOnly => {
                let mut content = self.content;
                content.children.append(&mut content.get_root_nodes());
                content.into()
            }
            RenderMode::LayoutOnly => {
                get_full_html_page(&self.config, self.title, {
                    let mut content = (self.layout)(Node {
                        identifier: Uuid::NAMESPACE_OID,
                        node_type: NodeType::Comment("VIEWY_CONTENT"),
                        text: None,
                        children: vec![],
                        class_list: Default::default(),
                        node_style: vec![],
                        attributes: HashMap::new(),
                        root_nodes: HashSet::new(),
                    });
                    content.children.append(&mut content.get_root_nodes());
                    content.into()
                }, theme_variant.to_string(), false)
            }
        }
    }
}