use viewy::components::*;
use viewy::{DefaultModifiers, sp, scale};

pub fn login() -> VStack {
    let mut o = VStack::new(Alignment::Start)
        .padding(vec![scale(6), scale(12)])
        .background_color("var(--background-raised)")
        .background_image("https://source.unsplash.com/random/1920x1080?social")
        .width("100%")
        .height("100%")
        .justify_content("center");
    o.append_child({
        let mut o = Card::new(CardStyle::Raised)
            .padding(vec![scale(6), scale(5)]);
        o.append_child(
            Text::new("Connexion", TextStyle::LargeTitle)
                .margin_bottom(16)
        );
        o.append_child({
            let mut o = Form::new("login-form", "/home")
                .width(&sp(300));
            o.append_child({
                let mut o = VStack::new(Alignment::Stretch)
                    .gap(vec![scale(3)]);
                o.append_child(TextField::new("email", FieldType::Email).label("Courriel"));
                o.append_child(TextField::new("password", FieldType::Password).label("Mot de passe"));
                o
            });
            o
        });
        o.append_child(
            Button::new("Connexion", ButtonStyle::Filled)
                .attach_to_form("login-form")
                .icon("log-in")
                .width("100%")
                .margin_top(scale(5))
        );
        o
    });
    o
}