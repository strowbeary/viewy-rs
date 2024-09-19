use crate::components::*;
use crate::node::*;
use std::collections::HashSet;
use std::{fmt, env};
use crate::Renderable;
use std::collections::HashMap;
use std::fmt::format;
use crate::config::Config;

fn get_full_html_page(title: String, content: String, theme_variant: String, insert_base_element: bool) -> String {
    let base_url = match env::var("BASE_URL") {
        Ok(url) => url,
        Err(_) => "".to_string()
    };
    let base_elem = {
        if insert_base_element {
            format!("<base href='{}/'>", base_url)
        } else {
            "".to_string()
        }
    };
    let config = Config::load();
    let favicons = config.app.favicons.iter()
        .map(|favicon| format!("<link rel=\"{rel}\" href=\"{base_url}{href}\">",
                               rel = favicon.rel,
                               base_url = base_url,
                               href = favicon.href
        ))
        .collect::<Vec<String>>()
        .join("");
    format!(r"
        <!doctype html>
        <html>
            <head>
                <title>{title}</title>
                <link rel='preconnect' href='https://rsms.me/'>
                {base_elem}
                {favicons}
                <link rel='stylesheet' href='https://rsms.me/inter/inter.css'>
                <link href='{base_url}/app.css' rel='stylesheet'>
                <script src='https://cdn.jsdelivr.net/npm/quill@2.0.2/dist/quill.js'></script>
                <link href='https://cdn.jsdelivr.net/npm/quill@2.0.2/dist/quill.snow.css' rel='stylesheet' />

                <script src='https://cdn.jsdelivr.net/npm/quill-blot-formatter@1.0.5/dist/quill-blot-formatter.min.js'></script>

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
            favicons = favicons,
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
            attributes: HashMap::new(),
            root_nodes: HashSet::new(),
        }
    }
}

/// Represent a page and embed all the logic required by the service worker.
///
/// # How to use
///
/// **Example to get the full page**
/// ```rust
/// use viewy::RenderMode;
/// Page::new("Page name", layout::Default, {
///     View::new()
/// })
///     .compile(RenderMode::Complete)
/// ```
#[derive(Clone)]
pub struct Page<'a> {
    name: String,
    content: Box<dyn Renderable>,
    layout: &'a dyn Fn(Box<dyn Renderable>) -> Box<dyn Renderable>,
    theme: Theme,
    base_url: bool,
}

impl<'a> Page<'a> {
    pub fn new<C: 'static + Renderable>(name: &str, layout: &'a dyn Fn(Box<dyn Renderable>) -> Box<dyn Renderable>, content: C) -> Self
    {
        Self {
            name: name.to_string(),
            content: Box::new(content),
            layout,
            theme: Theme::Auto,
            base_url: false,
        }
    }
    pub fn insert_base_element(&mut self) -> Self {
        self.base_url = true;
        self.clone()
    }
    pub fn compile(self, render_mode: RenderMode) -> String {

        let theme_variant = self.theme.to_string().to_lowercase();

        match render_mode {
            RenderMode::Complete => {
                get_full_html_page(self.name, {
                    (self.layout)(self.content).to_html()
                }, theme_variant, self.base_url)
            }
            RenderMode::ContentOnly => {
                self.content.to_html()
            }
            RenderMode::LayoutOnly => {
                get_full_html_page(self.name, {
                    (self.layout)(Box::new(ContentComment)).to_html()
                }, theme_variant, self.base_url)
            }
        }
    }
}