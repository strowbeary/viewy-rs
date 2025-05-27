use short_uuid::ShortUuid;
use std::default::Default;
use std::fmt::Write;
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
        Node {
            identifier: Uuid::new_v4(),
            node_type: NodeType::Normal("div"),
            text: None,
            children: vec![],
            class_list: HashSet::new(),
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

    pub fn render(self, html_buffer: &mut String) {
        let mut attributes = self.attributes;
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
        if !self.class_list.is_empty() {
            attributes.insert(
                "class".to_string(),
                Vec::from_iter(self.class_list).join(" "),
            );
        }
        match &self.node_type {
            NodeType::Normal(tag) => {
                write!(html_buffer, "<{}", tag).unwrap();
                for (k, v) in attributes {
                    write!(html_buffer, r#" {}="{}""#, k, v).unwrap();
                }
                html_buffer.push('>');
                if let Some(text_content) = self.text {
                    html_buffer.write_str(text_content.as_str()).unwrap();
                }
                for child in self.children {
                    child.render(html_buffer);
                }
                write!(html_buffer, "</{}>", tag).unwrap();
            }
            NodeType::SelfClosing(tag) => {
                write!(html_buffer, "<{}", tag).unwrap();
                for (k, v) in attributes {
                    write!(html_buffer, r#" {}="{}""#, k, v).unwrap();
                }
                write!(html_buffer, "/>").unwrap();
            }
            NodeType::Comment(comment) => {
                write!(html_buffer, "<!--{comment}-->").unwrap();
            }
        }
    }
}
