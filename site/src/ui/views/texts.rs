use viewy::prelude::*;

pub fn texts() -> VStack {
    let mut stack = VStack::new(Alignment::Stretch);
    stack
        .append_child(Text::new("LargeTitle", TextStyle::LargeTitle))
        .append_child(Text::new("H1", TextStyle::H1))
        .append_child(Text::new("H2", TextStyle::H2))
        .append_child(Text::new("H3", TextStyle::H3))
        .append_child(Text::new("Headline", TextStyle::Headline))
        .append_child(Text::new("Subtitle1", TextStyle::Subtitle1))
        .append_child(Text::new("Subtitle2", TextStyle::Subtitle2))
        .append_child(Text::new("Subtitle3", TextStyle::Subtitle3))
        .append_child(Text::new("Body", TextStyle::Body))
        .append_child(Text::new("Article", TextStyle::Article))
        .append_child(Text::new("Label", TextStyle::Label))
        .append_child(Text::new("Overline", TextStyle::Overline))
        .append_child(Text::new("Caption", TextStyle::Caption));
    stack
}
