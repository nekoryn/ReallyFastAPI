use sqlx::PgPool;
use crate::models::user::User;

pub struct UserRepository;

impl UserRepository {
    pub async fn get_all(db: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT id, name, age FROM users")
            .fetch_all(db)
            .await
    }
}