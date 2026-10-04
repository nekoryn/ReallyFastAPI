use really_fast_api::router::App;
use really_fast_api::response::Res;
use really_fast_api::error::{AppError, AppResult};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost:5432/postgres".to_string());

    // Подключаемся к реальной БД (убедись, что она запущена)
    let db_pool = PgPool::connect(&db_url).await.expect("Не удалось подключиться к PostgreSQL!");

    let mut app = App::new();

    // Регистрируем пул в расширениях приложения
    app.manage(db_pool);

    // 1. Успешный корень
    app.get("/", async |_req| {
        Ok(Res::ok_200("Hello from ReallyFastAPI server! 🚀 Try /db-error"))
    }).unwrap();

    // 2. Эндпоинт, вызывающий реальную ошибку БД через знак `?` (срабатывает From<sqlx::Error>)
    app.get("/db-error", async |req| -> AppResult<Res> {
        let db = req.get::<PgPool>().expect("DB pool not found");

        // Делаем запрос к несуществующей таблице, чтобы sqlx вернул ошибку
        let _val: (i64,) = sqlx::query_as("SELECT * FROM non_existent_table_12345")
            .fetch_one(&db)
            .await?; // Оператор `?` автоматически вызовет From::from для sqlx::Error!

        Ok(Res::ok_200("This won't be reached"))
    }).unwrap();

    println!("🚀 Server is running at http://127.0.0.1:8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}