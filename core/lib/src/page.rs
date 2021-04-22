use crate::renderer::{ToHtml, RenderableClone};
use crate::components::*;
use std::fmt;
use crate::Renderable;

fn get_full_html_page(title: String, content: String, theme_variant: String) -> String {
    format!(r"
        <!doctype html>
        <html>
        <head>
            <title>{title}</title>
            <link href='/app.css' rel='stylesheet'>
            <script src='https://unpkg.com/@popperjs/core@2'></script>
            <script src='/app.js'></script>
            <meta charset='utf8' />
            <meta name='viewport' content='width=device-width, initial-scale=1.0, user-scalable=no'>
            <meta name='apple-mobile-web-app-capable' content='yes'>
        </head>
        <body class='app-themes--{theme_variant}'>
            {content}
        </body>
        </html>
    ",
            title = title,
            content = content,
            theme_variant = theme_variant
    )
}

#[derive(Debug, Clone)]
pub enum Theme {
    Dark,
    Light,
    Auto,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub enum RenderMode {
    Complete,
    ContentOnly,
    /// Not used for the moment
    LayoutOnly
}

impl Clone for Box<dyn ToHtml> {
    fn clone(&self) -> Box<dyn ToHtml> {
        self.clone()
    }
}

#[derive(Clone)]
pub struct Page {
    name: String,
    content: Box<dyn ToHtml>,
    theme: Theme,
}

impl Page {
    pub fn new<C: 'static + ToHtml>(name: &str, content: C) -> Self {
        Self {
            name: name.to_string(),
            content: Box::new(content),
            theme: Theme::Auto,
        }
    }
    pub fn compile(&self, render_mode: RenderMode) -> String {
        let content = self.content.to_html();
        let theme_variant = self.theme.to_string().to_lowercase();
        match render_mode {
            RenderMode::Complete => {
                get_full_html_page(self.name.clone(), content, theme_variant)
            }
            RenderMode::ContentOnly => {
                content
            }
        }
    }
}