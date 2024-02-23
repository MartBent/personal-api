pub mod books;

use mongodb::{error::Error, options::ClientOptions, Client, Cursor};

const DATABASE_NAME: &str = "personal";

pub async fn get_document<T>(collection_name: &str) -> Result<Cursor<T>, Error> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await;
    let client = Client::with_options(client_options.unwrap()).unwrap();

    let database = client.database(DATABASE_NAME);
    let collection= database.collection::<T>(collection_name);

    collection.find(None, None).await
}