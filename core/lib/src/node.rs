use std::collections::HashSet;
use crate::components::Popover;

#[derive(Debug, Clone)]
pub enum NodeType {
    Normal(String),
    SelfClosing(String),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub text: Option<String>,
    pub children: Vec<Node>,
    pub class_list: HashSet<String>,
    pub node_style: Vec<(String, String)>,
    pub attributes: Vec<(String, String)>,
    pub popover: Box<Option<Popover>>,
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
            attributes: vec![],
            popover: Box::new(None),
        }
    }
}

impl Node {
    pub fn get_popovers(&self) -> Vec<Popover> {
        let mut popovers: Vec<Popover> = vec![];

        if let Some(popover) = self.popover.clone().take() {
            popovers.push(popover)
        }
        self.children.iter()
            .for_each(|child| {
                child.get_popovers().iter()
                    .for_each(|popover| popovers.push(popover.clone()))
            });
        popovers
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
            )
        }
    }
}

pub trait NodeContainer {
    fn get_node(&mut self) -> &mut Node;
}

