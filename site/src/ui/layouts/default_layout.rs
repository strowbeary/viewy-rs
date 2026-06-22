use viewy::{
    prelude::*,
    widgets::{
        image::Image,
        nav::{Nav, NavItem},
    },
};

pub fn default_layout<'a>() -> Layout<'a> {
    return &|content| {
        HStack::new(Alignment::Stretch)
            .height("100vh")
            .padding(vec![scale(6)])
            .gap(vec![scale(6)])
            .overflow(Overflow::Auto)
            .append_child(
                VStack::new(Alignment::Stretch)
                    .position(Position::Sticky)
                    .top("0")
                    .gap(vec![scale(6)])
                    .height("100%")
                    .min_width(&sp(200))
                    .append_child(HStack::new(Alignment::Center)
                        .gap(vec![scale(4)])
                        .append_child(Image::new("/assets/square-logo.svg").width(&sp(60)))
                        .append_child(Text::new("Viewy", TextStyle::H1)))
                    .append_child(
                        VStack::new(Alignment::Stretch)
                            .flex_grow(1)
                            .as_card(CardStyle::OutlinedRaised)
                            .append_child(
                                Nav::new(viewy::widgets::nav::NavOrientation::Vertical)
                                    .add_item(NavItem::new("Home", uri!(crate::home())).icon(Lucide::HouseHeart))
                                    .add_item(NavItem::new("Actions", uri!(crate::actions())).icon(Lucide::SquareDashedMousePointer))
                                    .add_item(NavItem::new(
                                        "Texts",
                                        uri!(crate::texts()),
                                    ).icon(Lucide::Type))     .add_item(NavItem::new(
                                        "Navigation",
                                        uri!(crate::nav_demo::nav_default()),
                                    ).icon(Lucide::Signpost))
                                    .add_item(NavItem::new(
                                        "Forms",
                                        uri!(crate::picker_select::picker_select_demo()),
                                    ).icon(Lucide::Form))
                                    .add_item(NavItem::new("Sheets", uri!(crate::sheet::sheet())).icon(Lucide::Layers2))
                                    .add_item(NavItem::new("Interactive Component", uri!(crate::interactive_component_poc::interactive_component_demo())).icon(Lucide::Component)),
                            ),
                    ),
            )
            .append_child(View::new().flex_grow(1).append_child(content))
            .into()
    };
}
