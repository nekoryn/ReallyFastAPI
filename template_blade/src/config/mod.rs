use sqlx::PgPool;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn init() -> Self {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());

        Self { database_url }
    }

    pub async fn connect_db(&self) -> PgPool {
        PgPool::connect(&self.database_url)
            .await
            .expect("Не удалось подключиться к базе данных!")
    }
}