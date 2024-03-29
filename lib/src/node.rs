use std::collections::{HashSet, HashMap};
use crate::components::{Popover, Popup};
use crate::engine::Renderable;

#[derive(Debug, Clone)]
pub enum NodeType {
    Normal(String),
    SelfClosing(String),
    Comment(String)
}

#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub text: Option<String>,
    pub children: Vec<Node>,
    pub class_list: HashSet<String>,
    pub node_style: Vec<(String, String)>,
    pub attributes: HashMap<String, String>,
    pub root_nodes: HashSet<Box<dyn Renderable>>,
}

impl Default for Node {
    fn default() -> Self {
        let mut class_list = HashSet::new();
        class_list.insert("view".to_string());
        Node {
            node_type: NodeType::Normal("div".to_string()),
            text: None,
            children: vec![],
            class_list,
            node_style: vec![],
            attributes: HashMap::new(),
            root_nodes: HashSet::new()
        }
    }
}

impl Node {
    pub fn get_root_nodes(&self) -> Vec<Box<dyn Renderable>> {
        let mut all_root_nodes: Vec<Box<dyn Renderable>> = Vec::from_iter(self.root_nodes.clone());

        self.children.iter()
            .for_each(|child| {
                all_root_nodes.append(&mut child.get_root_nodes())
            });
        all_root_nodes
    }


    pub fn get_html(&self) -> String {
        let classes: Vec<String> = self.class_list.iter()
            .map(|class_name| format!("{}", class_name))
            .collect();
        let attributes: Vec<String> = self.attributes.iter()
            .map(|(name, value)| format!("{}=\"{}\"", name, value))
            .collect();
        let style: Vec<String> = self.node_style.iter()
            .map(|(name, value)| format!("{}: {};", name, value))
            .collect();
        let content: Vec<String> = self.children.iter()
            .map(|child| child.get_html())
            .collect();
        match self.node_type.clone() {
            NodeType::Normal(tag_name) => format!(
                "<{tag} class=\"{class_list}\" {attributes} style=\"{view_style}\">{children}</{tag}>",
                tag = tag_name,
                class_list = classes.join(" "),
                attributes = attributes.join(" "),
                view_style = style.join(""),
                children = match self.text.clone() {
                    None => content.join(""),
                    Some(text) => text
                }
            ),
            NodeType::SelfClosing(tag_name) => format!(
                "<{tag} class=\"{class_list}\" {attributes} style=\"{view_style}\" />",
                tag = tag_name,
                class_list = classes.join(" "),
                attributes = attributes.join(" "),
                view_style = style.join("")
            ),
            NodeType::Comment(comment) => format!(
                "<!--{comment}-->",
                comment = comment
            )
        }
    }
}

pub trait NodeContainer {
    fn get_node(&mut self) -> &mut Node;
}


