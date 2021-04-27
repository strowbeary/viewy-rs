use viewy::*;
use viewy::components::*;
use std::ops::Deref;

pub fn default(content: Box<dyn Renderable>) -> Box<dyn Renderable> {
    Box::new(HStack::new(Alignment::Stretch)
        .height("100vh")
        .append_child({
            VStack::new(Alignment::Stretch)
                .height("100%")
                .min_width(sp(200).as_str())
                .gap(vec![24])
                .padding(vec![24])
                .background_color("var(--surface)")
                .append_child({
                    Menu::new(MenuStyle::VerticalNavbar)
                        .append_child({
                            MenuItem::new("Accueil")
                                .icon("home")
                                .action("/")
                        })
                        .append_child({
                            MenuItem::new("Tableau")
                                .icon("columns")
                                .action("/table")
                        })
                })
        })
        .append_child({
            VStack::new(Alignment::Stretch)
                .overflow(Overflow::Auto)
                .flex_grow(1)
                .append_child({
                    TitleBar::new("Viewy showcase")
                        .right_item({
                            Button::new("Login", ButtonStyle::Flat)
                                .icon("user")
                                .action("/login")
                        })
                        .bottom_item({
                            TextField::new("Search", FieldType::Search)
                                .placeholder("Search for everything")
                        })
                })
                .append_child({
                    View::new()
                        .padding(vec![16])
                        .append_child(content)
                })
        }))
}

pub fn login_layout(content: Box<dyn Renderable>) -> Box<dyn Renderable> {
    content
}