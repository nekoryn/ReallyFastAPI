use sqlx::PgPool;

pub struct Migrator;

impl Migrator {
    pub async fn run(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running database migrations...");
        sqlx::migrate!("./migrations").run(pool).await?;
        println!("Migrations applied successfully!");
        Ok(())
    }
}