use crate::components::*;
use crate::node::*;
use std::fmt;
use crate::Renderable;

fn get_full_html_page(title: String, content: String, theme_variant: String, base_url: &Option<String>) -> String {
    let base_elem = match base_url {
        Some(url) => format!("<base href='{}/'>", url),
        None => "".to_string()
    };
    let base_url = match base_url {
        Some(url) => url,
        None => "/".to_string()
    };
    format!(r"
        <!doctype html>
        <html>
        <head>
            <title>{title}</title>
            {base_elem}
            <link href='{base_url}/app.css' rel='stylesheet'>
            <script src='https://unpkg.com/@popperjs/core@2'></script>
            <script src='{base_url}/app.js'></script>
            <meta charset='utf-8' />
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
            theme_variant = theme_variant,
            base_elem = base_elem,
            base_url = base_url
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
    LayoutOnly,
}

#[derive(Clone, Debug)]
struct ContentComment;

impl Renderable for ContentComment {
    fn render(&self) -> Node {
        Node {
            node_type: NodeType::Comment("VIEWY_CONTENT".to_string()),
            text: None,
            children: vec![],
            class_list: Default::default(),
            node_style: vec![],
            attributes: vec![],
            popover: Box::new(None),
            popup: Box::new(None)
        }
    }
}

/// Represent a page and embed all the logic required by the service worker.
///
/// # How to use
///
/// **Example to get the full page**
/// ```rust
/// use viewy::page::RenderMode;
/// Page::new("Page name", layout::Default, {
///     View::new()
/// })
///     .compile(RenderMode::Complete)
/// ```
#[derive(Clone)]
pub struct Page<'a> {
    name: String,
    content: Box<dyn Renderable>,
    layout: &'a Fn(Box<dyn Renderable>) -> Box<dyn Renderable>,
    theme: Theme,
    base_url: Option<String>,
}

impl<'a> Page<'a> {
    pub fn new<C: 'static + Renderable>(name: &str, layout: &'a Fn(Box<dyn Renderable>) -> Box<dyn Renderable>, content: C) -> Self
    {
        Self {
            name: name.to_string(),
            content: Box::new(content),
            layout,
            theme: Theme::Auto,
            base_url: None,
        }
    }
    pub fn base_url(&mut self, base_url: &str) -> Self {
        self.base_url = Some(base_url.to_string());
        self.clone()
    }
    pub fn compile(&self, render_mode: RenderMode) -> String {
        let page = self;

        let theme_variant = page.theme.to_string().to_lowercase();
        match render_mode {
            RenderMode::Complete => {
                get_full_html_page(page.name.clone(), {
                    (page.layout)(page.content.clone()).to_html()
                }, theme_variant, &self.base_url)
            }
            RenderMode::ContentOnly => {
                page.content.to_html()
            }
            RenderMode::LayoutOnly => {
                get_full_html_page(page.name.clone(), {
                    (page.layout)(Box::new(ContentComment)).to_html()
                }, theme_variant, &self.base_url)
            }
        }
    }
}