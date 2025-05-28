use viewy::components::icons::Lucide;
use viewy::components::*;
use viewy::*;

pub fn navigation_and_menus() -> VStack {
    let mut stack = VStack::new(Alignment::Stretch);
    stack
        .padding(vec![scale(4)])
        .gap(vec![scale(3)])
        .append_child(Text::new("Navigation & menus", TextStyle::H1))
        .append_child(Divider::new())
        .append_child(
            Card::new(CardStyle::Raised)
                .padding(vec![scale(3)])
                .append_child(Text::new("Vertical menu", TextStyle::H1))
                .append_child(
                    Menu::new(MenuStyle::Vertical)
                        .append_child(MenuItem::new("Basic components").icon(Lucide::Box))
                        .append_child(
                            MenuItem::new("With a notification badge")
                                .icon(Lucide::Bell)
                                .badge(&12),
                        )
                        .append_child(MenuItem::new("Table").icon(Lucide::Columns3))
                        .append_child(MenuItem::new("Calendar").icon(Lucide::Calendar))
                        .append_child(MenuItem::new("Navigation & menus").icon(Lucide::Map)),
                ),
        )
        .append_child(
            Card::new(CardStyle::Raised)
                .padding(vec![scale(3)])
                .append_child(Text::new("Horizontal menu", TextStyle::H1))
                .append_child(
                    Menu::new(MenuStyle::HorizontalTab)
                        .append_child(MenuItem::new("Basic components").icon(Lucide::Box).popover(
                            {
                                let mut popover = Popover::new();
                                popover
                                    .placement(Placement::Bottom)
                                    .padding(vec![scale(3)])
                                    .append_child(Text::new("Hello", TextStyle::Body));
                                popover
                            },
                        ))
                        .append_child(MenuItem::new("Table").icon(Lucide::Columns3))
                        .append_child(
                            MenuItem::new("With a notification badge")
                                .icon(Lucide::Bell)
                                .badge(&12),
                        )
                        .append_child(MenuItem::new("Calendar").icon(Lucide::Calendar))
                        .append_child(MenuItem::new("Navigation & menus").icon(Lucide::Map)),
                ),
        )
        .append_child(
            Card::new(CardStyle::Raised)
                .padding(vec![scale(3)])
                .append_child(Text::new("Horizontal nav menu", TextStyle::H1))
                .append_child(
                    Menu::new(MenuStyle::HorizontalNav)
                        .append_child(MenuItem::new("Basic components").icon(Lucide::Box).popover(
                            {
                                let mut popover = Popover::new();
                                popover
                                    .placement(Placement::Bottom)
                                    .padding(vec![scale(3)])
                                    .append_child(Text::new("Hello", TextStyle::Body));
                                popover
                            },
                        ))
                        .append_child(MenuItem::new("Table").icon(Lucide::Columns3))
                        .append_child(MenuItem::new("Notifications").icon(Lucide::Bell).badge(&12))
                        .append_child(MenuItem::new("Calendar").icon(Lucide::Calendar))
                        .append_child(MenuItem::new("Navigation & menus").icon(Lucide::Map)),
                ),
        )
        .append_child(
            Card::new(CardStyle::Raised)
                .padding(vec![scale(3)])
                .append_child(Text::new("Menu with sections", TextStyle::H1))
                .append_child(
                    Menu::new(MenuStyle::Vertical)
                        .append_child(MenuSection::new("First Section"))
                        .append_child(MenuItem::new("Basic components").icon(Lucide::Box).popover(
                            {
                                let mut popover = Popover::new();
                                popover
                                    .placement(Placement::Bottom)
                                    .padding(vec![scale(3)])
                                    .append_child(Text::new("Hello", TextStyle::Body));
                                popover
                            },
                        ))
                        .append_child(MenuItem::new("Table").icon(Lucide::Columns3))
                        .append_child(MenuItem::new("Notifications").icon(Lucide::Bell).badge(&12))
                        .append_child(MenuSection::new("Second Section"))
                        .append_child(MenuItem::new("Table").icon(Lucide::Columns3))
                        .append_child(MenuItem::new("Calendar").icon(Lucide::Calendar))
                        .append_child(MenuItem::new("Navigation & menus").icon(Lucide::Map)),
                ),
        );
    stack
}