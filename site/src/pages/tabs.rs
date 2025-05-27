use viewy::components::{Alignment, Appendable, TabView, TabViewItem, Text, TextStyle, VStack};

pub fn tabs() -> VStack {
    VStack::new(Alignment::Stretch)
        .append_child({
            TabView::new()
                .append_child({
                    TabViewItem::new("Hello")
                        .append_child({
                            Text::new("Hello content", TextStyle::Body)
                        })
                })
                .append_child({
                    TabViewItem::new("Goodbye")
                        .append_child({
                            Text::new("Goodbye content", TextStyle::Body)
                        })
                })
        })
}