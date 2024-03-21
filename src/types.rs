use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    pub email: String,
    pub password: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserBody {
    pub email: String,
    pub password: String
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Message {
    pub sender: String,
    pub receiver: String,
    pub token: String,
}
