use rocket::serde::{Deserialize, Serialize};
use viewy::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, InteractiveComponentMessage)]
#[serde(crate = "rocket::serde")]
pub enum CounterMessage {
    Increment { amount: i64 },
    Decrement { amount: i64 },
    Reset,
}

#[derive(Debug, Clone, Serialize, Deserialize, InteractiveComponent)]
#[serde(crate = "rocket::serde")]
#[component(messages = CounterMessage)]
pub struct CounterComponent {
    pub value: i64,
}

impl viewy::prelude::InteractiveComponent for CounterComponent {
    type Message = CounterMessage;

    fn on_message(mut self, message: Self::Message) -> Self {
        match message {
            CounterMessage::Increment { amount } => self.value += amount,
            CounterMessage::Decrement { amount } => self.value -= amount,
            CounterMessage::Reset => self.value = 0,
        }
        self
    }

    fn render(self) -> Node {
        VStack::new(Alignment::Stretch)
            .add_class("counter-component")
            .gap(vec![scale(3)])
            .append_child(Text::new(
                &format!("Valeur courante: {}", self.value),
                TextStyle::H2,
            ))
            .append_child(
                HStack::new(Alignment::Center)
                    .gap(vec![scale(2)])
                    .append_child(Button::new("-5", ButtonStyle::Outlined).on_click(
                        Action::TriggerMessage(CounterMessage::Decrement { amount: 5 }),
                    ))
                    .append_child(Button::new("-1", ButtonStyle::Outlined).on_click(
                        Action::TriggerMessage(CounterMessage::Decrement { amount: 1 }),
                    ))
                    .append_child(Button::new("+1", ButtonStyle::Filled).on_click(
                        Action::TriggerMessage(CounterMessage::Increment { amount: 1 }),
                    ))
                    .append_child(Button::new("+5", ButtonStyle::Filled).on_click(
                        Action::TriggerMessage(CounterMessage::Increment { amount: 5 }),
                    ))
                    .append_child(
                        Button::new("Reset", ButtonStyle::Flat)
                            .on_click(Action::TriggerMessage(CounterMessage::Reset)),
                    ),
            )
            .into()
    }
}

#[get("/interactive-component-poc")]
pub fn interactive_component_demo() -> Page<'static> {
    Page::with_title("Viewy showcase – Interactive Component PoC").with_content({
        let mut main_stack = VStack::new(Alignment::Stretch);
        main_stack
            .gap(vec![scale(4)])
            .padding(vec![scale(5)])
            .append_child(Text::new(
                "PoC: composant interactif sans état serveur",
                TextStyle::H1,
            ))
            .append_child(Text::new(
                "Une seule route gère tous les composants interactifs. L'état du composant est transporté en HTML via data-v-component-state.",
                TextStyle::Body,
            ))
            .append_child(CounterComponent { value: 0 });
        main_stack
    })
}
