use viewy::components::{Alignment, Appendable, HStack, SanitizationLevel, TableOfContentItemType, TableOfContents, TableOfContentsItem, Text, TextStyle, VStack};
use viewy::{DefaultModifiers, Overflow, scale};

pub fn table_of_content() -> HStack {
    // Create the main content area
    let mut content = VStack::new(Alignment::Stretch);
    content.id("article-list");
    content.overflow(Overflow::Auto);

    // Create the table of contents
    let mut toc = TableOfContents::new("#article-list");

    // Add sections and chapters
    for section in 0..3 {
        let section_id = format!("section-{section}");
        
        // Add section to TOC
        toc.append_child(
            TableOfContentsItem::new(
                &format!("Section {section}"), 
                &format!("#{section_id}"), 
                TableOfContentItemType::H1
            )
        );
        
        // Add section title to content
        content.append_child(
            Text::new(&format!("Section {section}"), TextStyle::H1)
                .tag("h1")
                .id(&section_id)
        );
        
        // Add chapters for each section
        for chapter in 0..3 {
            let chapter_id = format!("{section_id}_chapter-{chapter}");
            
            // Add chapter to TOC
            toc.append_child(
                TableOfContentsItem::new(
                    &format!("Chapter {chapter}"), 
                    &format!("#{chapter_id}"), 
                    TableOfContentItemType::H2
                )
            );
            
            // Create chapter content with text
            let txt_content = include_str!("toc_content.txt").to_string();
            content.append_child(
                VStack::new(Alignment::Stretch)
                    .tag("section")
                    .id(&chapter_id)
                    .padding(vec![scale(4)])
                    .width("calc(2 * 100% / 3)")
                    .append_child(
                        Text::new(&format!("Chapter {chapter}"), TextStyle::H2)
                            .tag("h1")
                    )
                    .append_child(
                        Text::new(&txt_content.replace("\n", "<br>"), TextStyle::Body)
                            .sanitization_level(SanitizationLevel::Basic)
                    )
            );
        }
    }

    // Create the main horizontal stack
    let mut stack = HStack::new(Alignment::Stretch);
    stack.width("100%");
    stack.height("100%");
    
    // Create left panel for TOC
    let mut left_panel = VStack::new(Alignment::Stretch);
    left_panel.padding(vec![scale(4)]);
    left_panel.background_color("var(--surface-lighter)");
    left_panel.width("calc(100% / 3)");
    left_panel.append_child(toc);
    
    // Add both panels to the main stack
    stack.append_child(left_panel);
    stack.append_child(content);
    
    stack
}