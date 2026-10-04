use sqlx::PgPool;
use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;

pub struct UserService;

impl UserService {
    pub async fn fetch_users(db: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        // Здесь можно добавить бизнес-логику, кэширование или фильтрацию
        UserRepository::get_all(db).await
    }
}