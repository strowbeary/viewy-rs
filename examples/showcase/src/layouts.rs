use viewy::*;
use viewy::components::*;

pub fn default<C>(content: C) -> HStack
    where C: 'static + Renderable {
    HStack::new(Alignment::Stretch)
        .height("100vh")
        .append_child({
            VStack::new(Alignment::Stretch)
                .height("100%")
                .min_width(sp(200).as_str())
                .gap(vec![scale(5)])
                .padding(vec![scale(5)])
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
        })
}

pub fn login_layout<C>(content: C) -> C
    where C: 'static + Renderable {
    content
}