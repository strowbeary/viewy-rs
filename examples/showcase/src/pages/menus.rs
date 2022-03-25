use viewy::*;
use viewy::components::*;

pub fn navigation_and_menus() -> VStack {
    VStack::new(Alignment::Stretch)
        .gap(vec![scale(3)])
        .append_child({
            Text::new("Navigation & menus", TextStyle::H1)
        })
        .append_child({
            Divider::new()
        })
        .append_child({
            Card::new(CardStyle::Raised)
                .padding(vec![scale(3)])
                .append_child({
                    Text::new("Vertical menu", TextStyle::H1)
                })
                .append_child({
                    Menu::new(MenuStyle::Vertical)
                        .append_child({
                            MenuItem::new("Basic components")
                                .icon("box")
                        })
                        .append_child({
                            MenuItem::new("With a notification badge")
                                .icon("bell")
                                .badge(&12)
                        })
                        .append_child({
                            MenuItem::new("Table")
                                .icon("columns")
                        })
                        .append_child({
                            MenuItem::new("Calendar")
                                .icon("calendar")
                        })
                        .append_child({
                            MenuItem::new("Navigation & menus")
                                .icon("map")
                        })
                })
        })
        .append_child({
            Card::new(CardStyle::Raised)
                .padding(vec![scale(3)])
                .append_child({
                    Text::new("Horizontal menu", TextStyle::H1)
                })
                .append_child({
                    Menu::new(MenuStyle::HorizontalTab)
                        .append_child({
                            MenuItem::new("Basic components")
                                .icon("box")
                                .popover({
                                    Popover::new()
                                        .placement(Placement::Bottom)
                                        .padding(vec![scale(3)])
                                        .append_child({
                                            Text::new("Hello", TextStyle::Body)
                                        })
                                })
                        })
                        .append_child({
                            MenuItem::new("Table")
                                .icon("columns")
                        })
                        .append_child({
                            MenuItem::new("With a notification badge")
                                .icon("bell")
                                .badge(&12)
                        })
                        .append_child({
                            MenuItem::new("Calendar")
                                .icon("calendar")
                        })
                        .append_child({
                            MenuItem::new("Navigation & menus")
                                .icon("map")
                        })
                })
        })
        .append_child({
            Card::new(CardStyle::Raised)
                .padding(vec![scale(3)])
                .append_child({
                    Text::new("Horizontal nav menu", TextStyle::H1)
                })
                .append_child({
                    Menu::new(MenuStyle::HorizontalNav)
                        .append_child({
                            MenuItem::new("Basic components")
                                .icon("box")
                                .popover({
                                    Popover::new()
                                        .placement(Placement::Bottom)
                                        .padding(vec![scale(3)])
                                        .append_child({
                                            Text::new("Hello", TextStyle::Body)
                                        })
                                })
                        })
                        .append_child({
                            MenuItem::new("Table")
                                .icon("columns")
                        })
                        .append_child({
                            MenuItem::new("Notifications")
                                .icon("bell")
                                .badge(&12)
                        })
                        .append_child({
                            MenuItem::new("Calendar")
                                .icon("calendar")
                        })
                        .append_child({
                            MenuItem::new("Navigation & menus")
                                .icon("map")
                        })
                })
        })
        .append_child({
            Card::new(CardStyle::Raised)
                .padding(vec![scale(3)])
                .append_child({
                    Text::new("Menu with sections", TextStyle::H1)
                })
                .append_child({
                    Menu::new(MenuStyle::Vertical)
                        .append_child({
                            MenuSection::new("First Section")
                        })
                        .append_child({
                            MenuItem::new("Basic components")
                                .icon("box")
                                .popover({
                                    Popover::new()
                                        .placement(Placement::Bottom)
                                        .padding(vec![scale(3)])
                                        .append_child({
                                            Text::new("Hello", TextStyle::Body)
                                        })
                                })
                        })
                        .append_child({
                            MenuItem::new("Table")
                                .icon("columns")
                        })
                        .append_child({
                            MenuItem::new("Notifications")
                                .icon("bell")
                                .badge(&12)
                        })
                        .append_child({
                            MenuSection::new("Second Section")
                        })
                        .append_child({
                            MenuItem::new("Table")
                                .icon("columns")
                        })
                        .append_child({
                            MenuItem::new("Calendar")
                                .icon("calendar")
                        })
                        .append_child({
                            MenuItem::new("Navigation & menus")
                                .icon("map")
                        })
                })
        })
}