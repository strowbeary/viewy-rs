use viewy::components::*;
use viewy::*;

pub fn login_layout(content: Box<dyn Renderable>) -> Box<dyn Renderable> {
    Box::new({
        VStack::new(Alignment::Start)
            .padding(vec![scale(6), scale(12)])
            .background_color("var(--background-raised)")
            .background_image("https://source.unsplash.com/random/1920x1080?office")
            .width("100%")
            .height("100%")
            .justify_content("center")
            .append_child({
                Card::new(CardStyle::Raised)
                    .padding(vec![scale(5), scale(5), scale(6), scale(5)])
                    .append_child(content)
            })
    })
}