use rocket::serde::{Deserialize, Serialize};
use std::ops::Add;
use std::vec;
use viewy::prelude::*;

use crate::core::adapters::book_library_adapter::BookLibraryAdapter;
use crate::core::models::book::Book;
use crate::core::ports::book_library_port::BookLibraryPort;

#[derive(Debug, Clone, Serialize, Deserialize, InteractiveComponentMessage)]
#[serde(crate = "rocket::serde")]
pub enum PaginationMessage {
    NextPage,
    PreviousPage,
}

#[derive(Debug, Clone, Serialize, Deserialize, InteractiveComponent)]
#[serde(crate = "rocket::serde")]
#[component(messages = PaginationMessage)]
pub struct BookListPaginated {
    pub page_idx: usize,
    pub displayed_books: Vec<Book>,
    pub no_next_page: bool,
}

impl viewy::prelude::InteractiveComponent for BookListPaginated {
    type Message = PaginationMessage;

    fn on_message(mut self, message: Self::Message) -> Self {
        let library = BookLibraryAdapter::new();
        match message {
            PaginationMessage::NextPage => self.page_idx += 1,
            PaginationMessage::PreviousPage => {
                if self.page_idx > 0 {
                    self.page_idx -= 1;
                }
            }
        }
        self.displayed_books = library.list_books_paginated(self.page_idx, 10);
        self.no_next_page = library
            .list_books_paginated(self.page_idx + 1, 10)
            .is_empty();
        self
    }

    fn render(self) -> Node {
        VStack::new(Alignment::Stretch)
            .gap(vec![scale(5)])
            .append_child({
                let mut book_list = VStack::new(Alignment::Stretch);
                book_list.gap(vec![scale(4)]).flex_grow(1);
                for book in &self.displayed_books {
                    book_list.append_child(
                        HStack::new(Alignment::Center)
                            .as_card(CardStyle::OutlinedRaised)
                            .padding(vec![scale(4)])
                            .append_child(Text::new(&book.title, TextStyle::Body)),
                    );
                }
                book_list
            })
            .append_child(
                HStack::new(Alignment::Center)
                    .justify_content("center")
                    .gap(vec![scale(4)])
                    .append_child({
                        let mut prev_btn = Button::new("Prev", ButtonStyle::Filled);
                        prev_btn
                            .icon(Lucide::ChevronLeft)
                            .on_click(Action::TriggerMessage(PaginationMessage::PreviousPage));
                        if self.page_idx.eq(&0) {
                            prev_btn.disabled();
                        }
                        prev_btn
                    })
                    .append_child(Text::new(&self.page_idx.add(1).to_string(), TextStyle::H2))
                    .append_child({
                        let mut next_btn = Button::new("Next", ButtonStyle::Filled);
                        next_btn
                            .reverse()
                            .icon(Lucide::ChevronRight)
                            .on_click(Action::TriggerMessage(PaginationMessage::NextPage));
                        if self.displayed_books.len() == 0 || self.no_next_page {
                            next_btn.disabled();
                        }
                        next_btn
                    }),
            )
            .into()
    }
}

#[get("/interactive-component-poc")]
pub fn interactive_component_demo() -> Page<'static> {
    let library = BookLibraryAdapter::new();
    Page::with_title("Viewy showcase – Interactive Component PoC").with_content(
        VStack::new(Alignment::Stretch).gap(vec![scale(6)])
            .padding(vec![scale(6)])
            .append_child(
                VStack::new(Alignment::Stretch)
                    .gap(vec![scale(3)])
                    .append_child(Text::new(
                        "Composant interactif sans état serveur",
                        TextStyle::H1,
                    ))
                    .append_child(
                        Text::new(
                            "Une seule route gère tous les composants interactifs. L'état du composant est transporté en HTML via data-v-component-state.",
                            TextStyle::Body,
                        )
                    )
            )
            .append_child(BookListPaginated {page_idx:0,displayed_books:library.list_books_paginated(0,10), no_next_page: library
                .list_books_paginated(1, 10)
                .is_empty() })
    )
}
