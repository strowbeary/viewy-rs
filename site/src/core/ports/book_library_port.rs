use crate::core::models::book::Book;
pub enum Error {
    NotFound,
}
pub trait BookLibraryPort {
    fn add_book(&mut self, book: Book) -> Result<(), Error>;
    fn remove_book(&mut self, book_id: &str) -> Result<(), Error>;
    fn get_book(&self, book_id: &str) -> Result<&Book, Error>;
    fn list_books_paginated(&self, page: usize, per_page: usize) -> Vec<Book>;
}
