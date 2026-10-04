use tokio::task;
use tokio::sync::Semaphore;
use std::sync::Arc;
use std::time::Instant;
use really_fast_api::router::App;
use really_fast_api::response::Res;
use sqlx::PgPool;

#[tokio::test]
async fn stress_test_server_with_db() {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost:5432/postgres".to_string());

    let db_pool = PgPool::connect(&db_url)
        .await
        .expect("Не удалось подключится к PostgreSQL. Убедитесь что БД запущена!");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id BIGSERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )",
    )
    .execute(&db_pool)
    .await
    .unwrap();

    sqlx::query("TRUNCATE TABLE users RESTART IDENTITY")
        .execute(&db_pool)
        .await
        .ok();

    let mut app = App::new();

    app.manage(db_pool);

    app.get("/", async |_req| {
        Ok(Res::ok_200("Hello root"))
    }).unwrap();

    app.get("/user", |req| async move {
        let db = req.get::<PgPool>().expect("DB pool not found!");
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&db)
            .await
            .unwrap_or(0);

        Ok(Res::ok_200(&format!("Total users in DB: {}", count)))
    }).unwrap();

    app.post("/user", |req| async move {
        let db = req.get::<PgPool>().expect("DB pool not found");
        let _ = sqlx::query("INSERT INTO users (name, age) VALUES ($1, $2)")
            .bind("Alex")
            .bind(20)
            .execute(&db)
            .await;

        Ok(Res::created_201(&format!("User created with body: {}", req.body)))
    }).unwrap();

    app.put("/update/{id}", |req| async move {
        let db = req.get::<PgPool>().expect("DB pool not found");
        let unknown = "1".to_string();
        let id_val = req.params.get("id").unwrap_or(&unknown);
        let id: i64 = id_val.parse().unwrap_or(1);

        let _ = sqlx::query("UPDATE users SET name = $1 WHERE id = $2")
            .bind("UpdatedName")
            .bind(id)
            .execute(&db)
            .await;
        
        Ok(Res::ok_200(&format!("User {} updated!", id)))
    }).unwrap();

    app.delete("/user/{id}", |req| async move {
        let db = req.get::<PgPool>().expect("DB pool not found");
        let unknown = "1".to_string();
        let id_val = req.params.get("id").unwrap_or(&unknown);
        let id: i64 = id_val.parse().unwrap_or(1);

        let _ = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&db)
            .await;

        Ok(Res::ok_200(&format!("User {} deleted!", id)))
    }).unwrap();

    app.get("/users/{id}", async |req| {
        let unknown = "unknown".to_string();
        let id_val = req.params.get("id").unwrap_or(&unknown);
        let msg = format!("User ID: {}", id_val);
        Ok(Res::ok_200(&msg))
    }).unwrap();

    tokio::spawn(async move {
        let _ = app.listen("127.0.0.1:8082").await;
    });

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let client = reqwest::Client::builder()
        .http2_prior_knowledge()
        .build()
        .expect("Failed to build HTTP/2 client");
    let total_requests = 10_000;
    let concurrency_limit = 250;
    let semaphore = Arc::new(Semaphore::new(concurrency_limit));
    
    println!("Starting DB-backed stress test: {} requests (concurrency: {})...", total_requests, concurrency_limit);
    let start = Instant::now();

    let mut handles = vec![];

    for i in 0..total_requests {
        let client_clone = client.clone();
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        
        let handle = task::spawn(async move {
            let _permit = permit; // держим семафор до конца запроса
            
            let res = match i % 5 {
                0 => client_clone.get("http://127.0.0.1:8082/").send().await,
                1 => client_clone.get("http://127.0.0.1:8082/user").send().await,
                2 => client_clone.post("http://127.0.0.1:8082/user")
                        .header("Content-Type", "application/json")
                        .body(r#"{"name":"Alex","age":20}"#)
                        .send().await,
                3 => client_clone.put("http://127.0.0.1:8082/update/1")
                        .body("Update Data")
                        .send().await,
                _ => client_clone.delete("http://127.0.0.1:8082/user/42").send().await,
            };

            matches!(res, Ok(r) if r.status().is_success())
        });
        handles.push(handle);
    }

    let mut success_count = 0;
    for handle in handles {
        if let Ok(true) = handle.await {
            success_count += 1;
        }
    }

    let duration = start.elapsed();
    println!("Completed in: {:?}", duration);
    println!("Successful requests: {} / {}", success_count, total_requests);
    
    assert!(success_count > 9900, "Server dropped too many requests under load!");
}