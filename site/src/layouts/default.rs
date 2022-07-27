use viewy::*;
use viewy::components::*;

pub fn default_layout(content: Box<dyn Renderable>) -> Box<dyn Renderable> {
    Box::new(HStack::new(Alignment::Stretch)
        .height("100vh")
        .append_child({
            VStack::new(Alignment::Stretch)
                .height("100%")
                .min_width("max-content")
                .gap(vec![24])
                .padding(vec![24])
                .background_color("var(--surface)")
                .append_child({
                    Menu::new(MenuStyle::Vertical)
                        .append_child({
                            MenuItem::new("Basic components")
                                .icon("box")
                                .action("./")
                        })
                        .append_child({
                            MenuItem::new("Table")
                                .icon("columns")
                                .action("/table")
                        })
                        .append_child({
                            MenuItem::new("Calendar")
                                .icon("calendar")
                                .action("/calendar")
                        })
                        .append_child({
                            MenuItem::new("Navigation & menus")
                                .icon("map")
                                .action("/menus")
                        })
                        .append_child({
                            MenuItem::new("Forms")
                                .icon("edit")
                                .action("/forms")
                        })
                        .append_child({
                            MenuItem::new("Dynamic content")
                                .icon("search")
                                .action("/search")
                        })
                        .append_child({
                            MenuItem::new("Signature field")
                                .icon("edit-2")
                                .action("/signature")
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
                           HStack::new(Alignment::Center)
                               .append_child({
                                   Button::new("Sources", ButtonStyle::Flat)
                                       .icon("github")
                                       .action("https://github.com/strowbeary/viewy-rs")
                               })
                               .append_child({
                                   Button::new("Documentation", ButtonStyle::Flat)
                                       .icon("file-text")
                                       .action("https://docs.rs/crate/viewy/")
                               })
                        })
                })
                .append_child({
                    View::new()
                        .height("100%")
                        .width("100%")
                        .padding(vec![16])
                        .append_child(content)
                })
        }))
}