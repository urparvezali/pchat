use mongodb::{bson::{doc, raw::Error, Document}, options::ClientOptions, Client, Collection};

use crate::types::User;

const EMAIL: &str = "email";
const PASSWORD: &str = "password";
const SENDER: &str = "sender";
const RECEIVER: &str = "receiver";
const TOKEN: &str = "token";

pub async fn init() -> Result<Client, Error> {
    let db_url = "mongodb:localhost:27017";
    let client_opt = ClientOptions::parse(db_url).await.unwrap();
    let client = Client::with_options(client_opt).unwrap();
    Ok(client)
}
pub async fn query_user(client:&Client,usr:&User) -> Option<bool> {
	let users:Collection<Document> = client.database("pchat").collection("users");
	let query = doc! {EMAIL: usr.email.clone()};
	match users.find_one(query, None).await {
		Ok(_) => Some(true),
		Err(_) => Some(false),
	}
}