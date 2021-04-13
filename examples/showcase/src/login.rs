use viewy::components::*;
use viewy::{DefaultModifiers, sp, scale};
use viewy::component::Appendable;

pub fn login() -> VStack {
    VStack::new(Alignment::Start)
        .padding(vec![scale(6), scale(12)])
        .background_color("var(--background-raised)")
        .background_image("https://source.unsplash.com/random/1920x1080?medical")
        .width("100%")
        .height("100%")
        .justify_content("center")
        .append_child({
            Card::new(CardStyle::Raised)
                .padding(vec![scale(5), scale(5), scale(6), scale(5)])
                .append_child({
                    Button::new("Retour", ButtonStyle::Link)
                        .icon("arrow-left")
                        .action("/")
                })
                .append_child(
                    Text::new("Connexion", TextStyle::LargeTitle)
                        .margin_bottom(16)
                )
                .append_child({
                    Form::new("login-form", "/home")
                        .width(&sp(300))
                        .append_child({
                            VStack::new(Alignment::Stretch)
                                .gap(vec![scale(3)])
                                .append_child({
                                    TextField::new(
                                        "email",
                                        FieldType::Email,
                                    )
                                        .label("Courriel")
                                })
                                .append_child({
                                    TextField::new(
                                        "password",
                                        FieldType::Password,
                                    )
                                        .label("Mot de passe")
                                })
                        })
                })
                .append_child(
                    Button::new("Connexion", ButtonStyle::Filled)
                        .attach_to_form("login-form")
                        .icon("log-in")
                        .width("100%")
                        .margin_top(scale(5))
                )
        })
}