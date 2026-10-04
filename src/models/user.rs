use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub age: i32,
}