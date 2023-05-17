use viewy::components::{Alignment, Appendable, HStack, TableOfContents, TableOfContentsItem, Text, TextStyle, VStack};
use viewy::{DefaultModifiers, Overflow, scale};

pub fn table_of_content() -> HStack {
    let mut content = VStack::new(Alignment::Stretch)
        .id("article-list")
        .overflow(Overflow::Auto);
    let mut toc =  TableOfContents::new("#article-list");

    for i in 0..10 {
        toc.append_child({
            TableOfContentsItem::new(&format!("Title {i}"), &format!("#title-{i}"))
        });
        content.append_child({
            VStack::new(Alignment::Stretch)
                .tag("section")
                .id(&format!("title-{i}"))
                .padding(vec![scale(4)])
                .width("calc(2 * 100% / 3)")
                .append_child({
                    Text::new(&format!("Title {i}"), TextStyle::H1)
                        .tag("h1")
                })
                .append_child({
                    let txt_content = include_str!("toc_content.txt").to_string();
                    Text::new(&txt_content.replace("\n", "<br>"), TextStyle::Body)
                        .disable_purification()
                })
        });
    }

    HStack::new(Alignment::Stretch)
        .width("100%")
        .height("100%")
        .append_child({
            VStack::new(Alignment::Stretch)
                .padding(vec![scale(4)])
                .background_color("var(--surface-lighter)")
                .width("calc(100% / 3)")
                .append_child({
                    toc
                })
        })
        .append_child({
           content
        })
}