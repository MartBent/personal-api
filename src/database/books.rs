use log::error;
use futures::stream::TryStreamExt;
use crate::models::book::Book;
use super::get_document;

const COLLECTION_NAME: &str = "books";

pub async fn get_books() -> Vec<Book> {
    let books = get_document::<Book>(COLLECTION_NAME).await;
    match books {
        Ok(books_cursor) => {
            books_cursor.try_collect().await.unwrap_or_else(|_| vec![])
        },
        Err(error) => {
            error!("Error retrieving books {:?}", error);
            vec![]
        }
    }
}
