use viewy::components::{Alignment, Appendable, HStack, SanitizationLevel, TableOfContentItemType, TableOfContents, TableOfContentsItem, Text, TextStyle, VStack};
use viewy::{DefaultModifiers, Overflow, scale};

pub fn table_of_content() -> HStack {
    let mut content = VStack::new(Alignment::Stretch)
        .id("article-list")
        .overflow(Overflow::Auto);
    let mut toc =  TableOfContents::new("#article-list");
    for section in 0..3 {
        let section_id = format!("section-{section}");
        toc.append_child({
            TableOfContentsItem::new(&format!("Section {section}"), &format!("#{section_id}"), TableOfContentItemType::H1)
        });
        content.append_child({
            Text::new(&format!("Section {section}"), TextStyle::H1)
                .tag("h1")
                .id(&section_id)
        });
        for chapter in 0..3 {
            let chapter_id = format!("{section_id}_chapter-{chapter}");
            toc.append_child({
                TableOfContentsItem::new(&format!("Chapter {chapter}"), &format!("#{chapter_id}"), TableOfContentItemType::H2)
            });
            content.append_child({
                VStack::new(Alignment::Stretch)
                    .tag("section")
                    .id(&chapter_id)
                    .padding(vec![scale(4)])
                    .width("calc(2 * 100% / 3)")
                    .append_child({
                        Text::new(&format!("Chapter {chapter}"), TextStyle::H2)
                            .tag("h1")
                    })
                    .append_child({
                        let txt_content = include_str!("toc_content.txt").to_string();
                        Text::new(&txt_content.replace("\n", "<br>"), TextStyle::Body)
                            .sanitization_level(SanitizationLevel::Basic)
                    })
            });
        }
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