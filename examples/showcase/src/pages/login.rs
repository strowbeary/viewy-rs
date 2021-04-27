use viewy::components::*;
use viewy::{DefaultModifiers, sp, scale};

pub fn login() -> View {
    View::new()
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
            Form::new("login-form", "/login")
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
}