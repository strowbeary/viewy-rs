use viewy::*;
use viewy::components::*;
use viewy::components::icons::Lucide;

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
                                .icon(Lucide::Box)
                                .action("./")
                        })
                        .append_child({
                            MenuItem::new("Table")
                                .icon(Lucide::Columns)
                                .action("/table")
                        })
                        .append_child({
                            MenuItem::new("Calendar")
                                .icon(Lucide::Calendar)
                                .action("/calendar")
                        })
                        .append_child({
                            MenuItem::new("Navigation & menus")
                                .icon(Lucide::Map)
                                .action("/menus")
                        })
                        .append_child({
                            MenuItem::new("Forms")
                                .icon(Lucide::FormInput)
                                .action("/forms")
                        })
                        .append_child({
                            MenuItem::new("Dynamic content")
                                .icon(Lucide::Search)
                                .action("/search")
                        })
                        .append_child({
                            MenuItem::new("Signature field")
                                .icon(Lucide::FileSignature)
                                .action("/signature")
                        })
                        .append_child({
                            MenuItem::new("Table of content")
                                .icon(Lucide::ListTree)
                                .action("/table-of-content")
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
                                       .icon(Lucide::Github)
                                       .action("https://github.com/strowbeary/viewy-rs")
                               })
                               .append_child({
                                   Button::new("Documentation", ButtonStyle::Flat)
                                       .icon(Lucide::FileText)
                                       .action("https://docs.rs/viewy/latest/viewy/")
                               })
                        })
                })
                .append_child({
                    View::new()
                        .overflow(Overflow::Auto)
                        .height("100%")
                        .width("100%")
                        .append_child(content)
                })
        }))
}