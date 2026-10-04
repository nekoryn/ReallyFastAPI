use really_fast_api::{App, Res, Req, Migrator, Config};
use really_fast_api::error::AppResult;
use really_fast_api::get;
use really_fast_api::models::user::User;

#[get("/users")]
async fn get_users(req: Req) -> AppResult<Res> {
    let db = req.db();
    let users: Vec<User> = sqlx::query_as("SELECT id, name, age FROM users")
        .fetch_all(&db)
        .await?;

    Ok(Res::json(&users))
}

// Тестовый эндпоинт для проверки обеих таблиц
#[get("/seed")]
async fn seed_data(req: Req) -> AppResult<Res> {
    let db = req.db();

    // 1. Вставляем пользователя
    let user_row = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, age) VALUES ($1, $2) RETURNING id, name, age"
    )
    .bind("Alex")
    .bind(20)
    .fetch_one(&db)
    .await?;

    // 2. Вставляем пост для этого пользователя
    let _ = sqlx::query(
        "INSERT INTO posts (user_id, title, body) VALUES ($1, $2, $3)"
    )
    .bind(user_row.id)
    .bind("First Rust Post")
    .bind("Building our own framework is awesome!")
    .execute(&db)
    .await?;

    Ok(Res::ok_200(format!("Created user id {} and a post!", user_row.id)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::init();
    let db_pool = cfg.connect_db().await;

    // Автоматические миграции (накатит обе!)
    Migrator::run(&db_pool).await?;

    let mut app = App::new();
    app.manage(db_pool);

    app.register_collected().unwrap();

    println!("🚀 Server is running at http://127.0.0.1:8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}