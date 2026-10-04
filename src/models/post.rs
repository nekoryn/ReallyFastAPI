use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub body: String,
}