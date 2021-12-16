use std::collections::HashMap;

use crate::{DefaultModifiers, sp};
use crate::Renderable;
use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone)]
pub enum Size {
    Normal,
    Large,
    XLarge,
}

#[derive(Debug, Clone)]
pub struct Avatar {
    node: Node,
    name: String,
    profil_img: Option<String>,
    size: Size,
}

impl Avatar {
    pub fn new(name: &str, profil_img: &Option<String>) -> Self {
        Self {
            node: Default::default(),
            name: name.to_string(),
            profil_img: profil_img.clone(),
            size: Size::Normal,
        }
    }

    pub fn size(&mut self, size: Size) -> Self {
        self.size = size;
        self.clone()
    }
}

impl NodeContainer for Avatar {
    fn get_node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl DefaultModifiers<Avatar> for Avatar {}

impl Renderable for Avatar {
    fn render(&self) -> Node {
        let mut avatar = self.clone()
            .add_class("avatar")
            .add_class(&format!("avatar--{}", match self.size {
                Size::Normal => "normal",
                Size::Large => "large",
                Size::XLarge => "x-large",
            }));
        if let Some(profile_img) = self.profil_img.clone() {
            avatar = avatar.background_image(&profile_img);
        } else {
            let initials: Vec<char> = self.name
                .clone()
                .split(" ")
                .filter(|word| {
                    !vec!["le", "de", "des", "la", "les"].contains(&word.to_lowercase().as_str())
                })
                .map(|part| part.chars().nth(0).unwrap_or_default())
                .collect();
            let text_content: String = initials[0..2].iter().collect();
            avatar.get_node().text = Some(text_content.to_uppercase());
        }
        avatar.get_node().clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::components::Avatar;
    use crate::Renderable;

    #[test]
    fn test_different_names() {
        let names = vec![
            "Rémi Caillot",
            "Estelle Le Marre",
            "Jean-François Cano",
        ];
        for name in names {
            let avatar = Avatar::new(name, &None);
            println!("{} => {}", name, avatar.to_html())
        }
    }
}