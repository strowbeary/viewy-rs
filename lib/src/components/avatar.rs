use std::ops::Deref;
use std::ops::DerefMut;

use crate::DefaultModifiers;
use crate::Renderable;
use crate::node::Node;

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

    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = size;
        self
    }
}

impl DefaultModifiers for Avatar {}

impl Deref for Avatar {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl DerefMut for Avatar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl Renderable for Avatar {
    fn render(mut self) -> Node {
        let size = match &self.size {
            Size::Normal => "normal",
            Size::Large => "large",
            Size::XLarge => "x-large",
        };
        self.add_class("avatar")
            .add_class(&format!("avatar--{size}"));
        if let Some(img_profile) = self.profil_img.clone() {
            self.background_image(&img_profile);
        } else {
            let initials: Vec<char> = self
                .name
                .split(" ")
                .filter(|word| {
                    !vec!["le", "de", "des", "la", "les"].contains(&word.to_lowercase().as_str())
                })
                .map(|part| part.chars().nth(0).unwrap_or_default())
                .collect();
            let text_content = initials
                .get(0..2)
                .map(|initials| -> String { initials.iter().collect() })
                .unwrap_or(self.name.get(0..2).unwrap_or_default().to_string());
            self.node.text = Some(text_content.to_uppercase());
        }
        self.node
    }
}

#[cfg(test)]
mod tests {
    use crate::Renderable;
    use crate::components::Avatar;

    #[test]
    fn test_different_names() {
        let names = vec![
            "Rémi Caillot",
            "Estelle Le Marre",
            "Jean-François Cano",
            "remicaillot5@gmail.com",
        ];
        for name in names {
            let avatar = Avatar::new(name, &None).render();
            println!("{} => {}", name, avatar.to_html())
        }
    }
}
