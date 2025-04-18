use rayon::prelude::*;
use short_uuid::ShortUuid;
use std::default::Default;
use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum NodeType {
    Normal(&'static str),
    SelfClosing(&'static str),
    Comment(&'static str),
}

/// Represent HTML DOM node that will be generated on render
/// Every components will configure one or many nodes.
#[derive(Clone, Debug)]
pub struct Node {
    pub identifier: Uuid,
    pub node_type: NodeType,
    pub text: Option<String>,
    pub children: Vec<Node>,
    pub class_list: HashSet<String>,
    pub node_style: Vec<(String, String)>,
    pub attributes: HashMap<String, String>,
    pub root_nodes: HashSet<Node>,
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.identifier.eq(&other.identifier)
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
    }
}

impl Default for Node {
    fn default() -> Self {
        let mut class_list = HashSet::new();
        class_list.insert("view".to_string());
        Node {
            identifier: Uuid::new_v4(),
            node_type: NodeType::Normal("div"),
            text: None,
            children: vec![],
            class_list,
            node_style: vec![],
            attributes: HashMap::default(),
            root_nodes: HashSet::default(),
        }
    }
}

impl Node {
    pub fn get_root_nodes(&self) -> Vec<Node> {
        let mut root_nodes: Vec<Node> = Vec::from_iter(self.root_nodes.iter().cloned());
        let mut children_root_nodes = self
            .children
            .iter()
            .map(|child| child.get_root_nodes())
            .flatten()
            .collect::<Vec<Node>>();
        root_nodes.append(&mut children_root_nodes);
        root_nodes
    }

    pub fn get_node_style(&self) -> Option<(String, String)> {
        if !self.node_style.is_empty() {
            let short_identifier = ShortUuid::from_uuid(&self.identifier).to_string();
            let concat_property = self
                .node_style
                .iter()
                .map(|(prop_name, prop_value)| format!("    {prop_name}: {prop_value};"))
                .collect::<Vec<String>>()
                .join("\n");
            Some((
                short_identifier.clone(),
                format!(
                    "\
.{short_identifier} {{
{concat_property}
}}\n"
                ),
            ))
        } else {
            None
        }
    }
}

/// HtmlCssJs type describe the 3 components for a dynamic page :
/// HTML code, CSS code, JavaScript code
#[derive(Default, Clone, Debug)]
pub struct HtmlCssJs {
    pub html: String,
    pub css: String,
    pub js: String,
}

impl Into<HtmlCssJs> for Node {
    fn into(mut self) -> HtmlCssJs {
        let style = self.get_node_style().unwrap();
        let css_style = if let Some((node_class_name, node_style)) = self.get_node_style() {
            self.class_list.insert(node_class_name);
            node_style
        } else {
            String::new()
        };
        let mut attributes = self.attributes;

        if !self.class_list.is_empty() {
            attributes.insert(
                "class".to_string(),
                Vec::from_iter(self.class_list).join(" "),
            );
        }

        if !self.node_style.is_empty() {
            attributes.insert(
                "style".to_string(),
                self.node_style
                    .iter()
                    .map(|(name, value)| format!("{}: {};", name, value))
                    .collect::<Vec<String>>()
                    .join(""),
            );
        }

        let attributes: Vec<String> = attributes
            .iter()
            .map(|(name, value)| format!("{}=\"{}\"", name, value))
            .collect();

        let content: Vec<HtmlCssJs> = self
            .children
            .into_par_iter()
            .map(|node| node.into())
            .collect();

        let html = match self.node_type.clone() {
            NodeType::Normal(tag_name) => format!(
                "<{tag_name} {attributes}>{children}</{tag_name}>",
                attributes = attributes.join(" "),
                children = match self.text.clone() {
                    None => content
                        .into_iter()
                        .map(|HtmlCssJs { html, .. }| html)
                        .collect::<Vec<String>>()
                        .join(""),
                    Some(text) => text,
                }
            ),
            NodeType::SelfClosing(tag_name) => format!(
                "<{tag_name} {attributes}/>",
                attributes = attributes.join(" "),
            ),
            NodeType::Comment(comment) => format!("<!--{comment}-->"),
        };
        HtmlCssJs {
            html,
            css: css_style,
            js: String::new(),
        }
    }
}

impl FromIterator<Node> for Vec<HtmlCssJs> {
    fn from_iter<T: IntoIterator<Item = Node>>(iter: T) -> Self {
        let mut collection = Self::new();
        for node in iter {
            collection.push(node.into());
        }
        collection
    }
}

impl From<Vec<HtmlCssJs>> for HtmlCssJs {
    fn from(value: Vec<HtmlCssJs>) -> Self {
        value.into_iter().fold(HtmlCssJs::default(), |mut a, b| {
            a.html.push_str(&b.html);
            a.css.push_str(&b.css);
            a.js.push_str(&b.js);
            return a;
        })
    }
}
