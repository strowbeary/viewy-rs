use viewy::components::*;
use viewy::{DefaultModifiers, scale};

pub fn signature_field() -> VStack {
    VStack::new(Alignment::Stretch)
        .padding(vec![scale(4)])
        .append_child({
            Text::new("Signature field", TextStyle::H1)
        })
        .append_child({
            Divider::new()
        })
        .append_child({
            SignatureField::new("signature")
                .label("Signature du client")
        })
}