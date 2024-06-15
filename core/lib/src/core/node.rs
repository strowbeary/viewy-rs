use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::task::{Context, Poll};
use uuid::Uuid;
use rayon::prelude::*;
use rocket::futures::Stream;
use rocket::response::stream::ReaderStream;

#[derive(Clone, Debug)]
pub enum NodeType {
    Normal(&'static str),
    SelfClosing(&'static str),
    Comment(&'static str)
}

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
            attributes: Default::default(),
            root_nodes: Default::default(),
        }
    }
}


impl Node {
    pub fn get_root_nodes(&self) -> Vec<Node> {
        let mut root_nodes: Vec<Node> = Vec::from_iter(self.root_nodes.iter().cloned());
        let mut children_root_nodes = self.children.iter()
            .map(|child| {
                child.get_root_nodes()
            }).flatten().collect::<Vec<Node>>();
        root_nodes.append(&mut children_root_nodes);
        root_nodes
    }

}

impl Into<String> for Node {
    fn into(self) -> String {
        let mut attributes = self.attributes;
        if !self.class_list.is_empty() {
            attributes.insert("class".to_string(), Vec::from_iter(self.class_list).join(" "));

        }

        if !self.node_style.is_empty() {
            attributes.insert("style".to_string(),self.node_style.iter()
                .map(|(name, value)| format!("{}: {};", name, value))
                .collect::<Vec<String>>()
                .join(""));

        }

        let attributes: Vec<String> = attributes.iter()
            .map(|(name, value)| format!("{}=\"{}\"", name, value))
            .collect();

        let content: Vec<String> = self.children.into_par_iter().map(|node| node.into()).collect();

        match self.node_type.clone() {
            NodeType::Normal(tag_name) => format!(
                "<{tag_name} {attributes}>{children}</{tag_name}>",
                attributes = attributes.join(" "),
                children = match self.text.clone() {
                    None => content.join(""),
                    Some(text) => text
                }
            ),
            NodeType::SelfClosing(tag_name) => format!(
                "<{tag_name} {attributes}/>",

                attributes = attributes.join(" "),
            ),
            NodeType::Comment(comment) => format!(
                "<!--{comment}-->"
            )
        }
    }
}

impl FromIterator<Node> for Vec<String> {
    fn from_iter<T: IntoIterator<Item=Node>>(iter: T) -> Self {
        let mut collection = Self::new();
        for node in iter {
            collection.push(node.into());
        }
        collection
    }
}
