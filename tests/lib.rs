use viewy::{StyleRegistery, Component};
use viewy::components::*;
use view::view;

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

    let component = Component::<UserProfile>(|state| {
        return view! {
            VStack {
                VStack
            }
        }
    });

    println!("{:?}", component.compile(state));
}