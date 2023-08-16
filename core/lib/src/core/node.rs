use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use uuid::Uuid;

#[derive(Clone)]
pub enum NodeType {
    Normal(&'static str),
    SelfClosing(&'static str),
    Comment(&'static str)
}

#[derive(Clone)]
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
        let mut all_root_nodes: Vec<Node> = Vec::from_iter(self.root_nodes.iter().cloned());

        self.children.iter()
            .for_each(|child| {
                all_root_nodes.append(&mut child.get_root_nodes())
            });
        all_root_nodes
    }

}

impl Into<String> for Node {
    fn into(self) -> String {
        let classes: Vec<String> = self.class_list.iter()
            .map(|class_name| format!("{}", class_name))
            .collect();
        let attributes: Vec<String> = self.attributes.iter()
            .map(|(name, value)| format!("{}=\"{}\"", name, value))
            .collect();
        let style: Vec<String> = self.node_style.iter()
            .map(|(name, value)| format!("{}: {};", name, value))
            .collect();
        let content: Vec<String> = self.children.into_iter()
            .collect();
        match self.node_type.clone() {
            NodeType::Normal(tag_name) => format!(
                "<{tag} class=\"{class_list}\" style=\"{view_style}\" {attributes}>{children}</{tag}>",
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

impl FromIterator<Node> for Vec<String> {
    fn from_iter<T: IntoIterator<Item=Node>>(iter: T) -> Self {
        let mut collection = Self::new();
        for node in iter {
            collection.push(node.into());
        }
        collection
    }
}


