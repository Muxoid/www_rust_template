use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 0, // Assuming 0 is an "empty" value for id
            name: "".to_string(),
            email: "".to_string(),
            password_hash: "".to_string(),
        }
    }
}
