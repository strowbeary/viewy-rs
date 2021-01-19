use viewy_rs::{StyleRegistery, View, view, Component};
use viewy_rs::components::*;

struct UserProfile {
    pub name: String,
    pub email: String
}

#[test]
pub fn create_view() {
    let state = UserProfile{
        name: "Rémi Caillot".to_string(),
        email: "remicaillot5@gmail.com".to_string()
    };

    let component = Component::<UserProfile, Card>(|state| {
        return view! {
            Card(style: CardStyle::Filled) {
                Button(label: state.name, style: ButtonStyle::Link)
                    .destructive()
                    .disabled(true)
            }
        }
    });

    println!("{:?}", component.compile(state));
}