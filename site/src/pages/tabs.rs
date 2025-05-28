use viewy::components::{Alignment, Appendable, TabView, TabViewItem, Text, TextStyle, VStack};

pub fn tabs() -> VStack {
    let mut stack = VStack::new(Alignment::Stretch);

    let hello_text = Text::new("Hello content", TextStyle::Body);
    let mut hello_tab = TabViewItem::new("Hello");
    hello_tab.append_child(hello_text);

    let goodbye_text = Text::new("Goodbye content", TextStyle::Body);
    let mut goodbye_tab = TabViewItem::new("Goodbye");
    goodbye_tab.append_child(goodbye_text);

    let mut tab_view = TabView::new();
    tab_view.append_child(hello_tab);
    tab_view.append_child(goodbye_tab);

    stack.append_child(tab_view);
    stack
}
