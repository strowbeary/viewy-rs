use std::collections::{HashMap};
use uuid::Uuid;

use crate::core::config::Config;
use crate::core::layout::Layout;
use crate::core::theme::Theme;
use crate::node::{Node, NodeType};
use crate::core::page::html_page::get_full_html_page;

mod html_page;

/// `RenderMode` enum is used to determine how to render a `Page`.
#[derive(Debug)]
pub enum RenderMode {
    /// This mode will result in a complete HTML page, with the page content wrapped within the page's `Layout`.
    Complete,
    /// This mode will just render the page content without any layout.
    ContentOnly,
    /// Not used for the moment
    LayoutOnly,
}



pub struct Page<'a> {
    pub content: Node,
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
    where
        C: Into<Node>,
    {
        self.content = content.into();
        self
    }

    pub fn with_layout(mut self, layout: Layout<'a>) -> Self {
        self.layout = layout.into();
        self
    }

    pub fn compile(self, render_mode: RenderMode) -> String {
        let theme_variant = self.theme.as_str();
        let mut html_buffer = String::new();
        match render_mode {
            RenderMode::Complete => get_full_html_page(
                &self.config,
                self.title,
                {
                    let content = (self.layout)(self.content);

                    content.render(&mut html_buffer);
                    html_buffer
                },
                theme_variant.to_string(),
                false,
            ),
            RenderMode::ContentOnly => {
                self.content.render(&mut html_buffer);
                html_buffer
            }
            RenderMode::LayoutOnly => get_full_html_page(
                &self.config,
                self.title,
                {
                    let mut content = (self.layout)(Node {
                        identifier: Uuid::NAMESPACE_OID,
                        node_type: NodeType::Comment("VIEWY_CONTENT"),
                        text: None,
                        children: vec![],
                        class_list: Default::default(),
                        node_style: vec![],
                        attributes: HashMap::new(),
                    });

                    content.render(&mut html_buffer);
                    html_buffer
                },
                theme_variant.to_string(),
                false,
            ),
        }
    }
}
