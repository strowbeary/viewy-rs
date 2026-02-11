use uuid::Uuid;

use crate::core::config::Config;
use crate::core::layout::Layout;
use crate::core::page::html_page::get_full_html_page;
use crate::core::theme::Theme;
use crate::node::{Node, NodeType};
use crate::widgets::icon::icons::sprite_from_icon_ids;
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::vec;

mod html_page;

pub trait HtmlStream: Stream<Item = String> + Send {}
impl<T> HtmlStream for T where T: Stream<Item = String> + Send {}

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

fn collect_used_icon_ids(node: &Node, icon_ids: &mut Vec<String>) {
    if let Some(icon_id) = node.attributes.get("data-v-icon-id") {
        icon_ids.push(icon_id.clone());
    }

    for child in &node.children {
        collect_used_icon_ids(child, icon_ids);
    }
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

    pub fn render_stream(
        self,
        render_mode: RenderMode,
    ) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(futures::stream::iter(vec![]))
    }

    pub fn compile(self, render_mode: RenderMode) -> String {
        let theme_variant = self.theme.as_str();
        let mut html_buffer = String::new();
        match render_mode {
            RenderMode::Complete => {
                let content = (self.layout)(self.content);
                let mut icon_ids = vec![];
                collect_used_icon_ids(&content, &mut icon_ids);
                let sprite = sprite_from_icon_ids(icon_ids.iter().map(|id| id.as_str()));

                content.render(&mut html_buffer);
                get_full_html_page(
                    &self.config,
                    self.title,
                    sprite,
                    html_buffer,
                    theme_variant.to_string(),
                    false,
                )
            }
            RenderMode::ContentOnly => {
                let mut icon_ids = vec![];
                collect_used_icon_ids(&self.content, &mut icon_ids);
                let sprite = sprite_from_icon_ids(icon_ids.iter().map(|id| id.as_str()));

                self.content.render(&mut html_buffer);
                format!("{sprite}{html_buffer}")
            }
            RenderMode::LayoutOnly => {
                let content = (self.layout)(Node {
                    identifier: Uuid::NAMESPACE_OID,
                    node_type: NodeType::Comment("VIEWY_CONTENT"),
                    ..Node::default()
                });
                let mut icon_ids = vec![];
                collect_used_icon_ids(&content, &mut icon_ids);
                let sprite = sprite_from_icon_ids(icon_ids.iter().map(|id| id.as_str()));

                content.render(&mut html_buffer);
                get_full_html_page(
                    &self.config,
                    self.title,
                    sprite,
                    html_buffer,
                    theme_variant.to_string(),
                    false,
                )
            }
        }
    }
}
