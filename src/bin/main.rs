use really_fast_api::router::App;
use really_fast_api::response::Res;
use really_fast_api::request::Req;
use really_fast_api::error::AppResult;
use really_fast_api::get;
use sqlx::PgPool;


#[get("/")]
async fn handle_root(_req: Req) -> AppResult<Res> {
    Ok(Res::ok_200("Hello from ReallyFastAPI server! 🚀 Try /db-error"))
}

#[get("/db-error")]
async fn handle_db_error(req: Req) -> AppResult<Res> {
    let db = req.get::<PgPool>().expect("DB pool not found");

    let _val: (i64,) = sqlx::query_as("SELECT * FROM non_existent_table_12345")
        .fetch_one(&db)
        .await?; 

    Ok(Res::ok_200("This won't be reached"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost:5432/postgres".to_string());

    let db_pool = PgPool::connect(&db_url).await.expect("Не удалось подключиться к PostgreSQL!");

    let mut app = App::new();
    app.manage(db_pool);

    app.register_collected().unwrap();

    println!("🚀 Server is running at http://127.0.0.1:8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}