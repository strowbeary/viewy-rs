use std::borrow::BorrowMut;
use crate::{DefaultModifiers, Renderable};
use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone)]
pub enum SanitizationLevel {
    /// No sanitization is performed.
    None,
    /// Only basic HTML tags are allowed.
    /// Use this level if you need to render content issued from RichTextArea
    Basic,
    /// Default level.
    /// No HTML tags allowed, only text
    Strict,
}

/// Used to set the typographic style of a Text view.
#[derive(Debug, Clone)]
pub enum TextStyle {
    LargeTitle,
    H1,
    H2,
    H3,
    Headline,
    Subtitle1,
    Subtitle2,
    Subtitle3,
    Body,
    Article,
    Button,
    Label,
    Overline,
    Caption,
}

/// A view that displays one or more lines of read-only text.
#[derive(Debug, Clone)]
pub struct Text {
    node: Node,
    pub content: String,
    pub style: TextStyle,
    pub no_wrap: bool,
    pub sanitization_level: SanitizationLevel,
}

impl Text {
    pub fn new(content: &str, style: TextStyle) -> Self {
        Text {
            node: Node::default(),
            content: content.to_string(),
            style,
            no_wrap: false,
            sanitization_level: SanitizationLevel::Strict,
        }
    }
    pub fn bold(&mut self, is_bold: bool) -> Self {
        if is_bold {
            self.get_node().node_style.push(("font-weight".to_string(), "bold".to_string()));
        } else {
            self.get_node().node_style.push(("font-weight".to_string(), "normal".to_string()));
        }
        self.clone()
    }
    pub fn uppercase(&mut self, is_uppercase: bool) -> Self {
        if is_uppercase {
            self.get_node().node_style.push(("text-transform".to_string(), "uppercase".to_string()));
        } else {
            self.get_node().node_style.push(("text-transform".to_string(), "none".to_string()));
        }
        self.clone()
    }


    pub fn no_wrap(&mut self, is_no_wrap: bool) -> Self {
        self.no_wrap = is_no_wrap;
        self.clone()
    }
    
    #[deprecated(since = "0.13.16", note = "Use sanitization_level instead")]
    pub fn disable_purification(&mut self) -> Self {
        self.sanitization_level = SanitizationLevel::Basic;
        self.clone()
    }

    /// Define sanitization level
    pub fn sanitization_level(&mut self, level: SanitizationLevel) -> Self {
        self.sanitization_level = level;
        self.clone()
    }

    pub fn text_overflow(&mut self) -> Self {
        self.get_node().node_style.push(("text-overflow".to_string(), "ellipsis".to_string()));
        self.clone()
    }
    pub fn text_shadow(&mut self, rule: &str) -> Self {
        self.get_node().node_style.push(("text-shadow".to_string(), rule.to_string()));
        self.clone()
    }
}

impl NodeContainer for Text {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Text> for Text {}

impl Renderable for Text {
    fn render(&self) -> Node {
        let mut text = self.clone()
            .add_class("text")
            .add_class(format!("text--{:?}", self.style).to_lowercase().as_str());

        match self.sanitization_level {
            SanitizationLevel::None => {
                text.node.text = Some(self.content.to_string())
            }
            SanitizationLevel::Basic => {
                text.node.text = Some(ammonia::clean(&self.content))
            }
            SanitizationLevel::Strict => {
                text.node.text = Some(ammonia::Builder::empty()
                    .clean(&self.content)
                    .to_string())
            }
        }

        if self.no_wrap {
            text.add_class("text--nowrap");
        }
        text.node
    }
}