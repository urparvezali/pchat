use axum::response::IntoResponse;
use axum::{extract::State, response::Html, Form};
use mongodb::Client;
use std::fs::read_to_string;

use crate::db::query_user;
use crate::types::User;

pub async fn root() -> Html<String> {
    let content = Html(read_to_string("templates/root.html").unwrap_or_default());
    content
}
pub async fn auth(State(client): State<Client>, Form(usr): Form<User>) -> impl IntoResponse {
	let page = read_to_string("templates/chat.html").unwrap_or("Page not available".to_string());
    if query_user(&client, &usr).await.unwrap() {
		return Html(page);
	}

	Html(page)
}
