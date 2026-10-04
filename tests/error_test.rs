use really_fast_api::router::App;
use really_fast_api::response::Res;
use really_fast_api::request::Req;
use really_fast_api::error::{AppError, AppResult};
use really_fast_api::get;

#[get("/not-found-test")]
async fn handle_not_found(_req: Req) -> AppResult<Res> {
    Err(AppError::NotFound("Custom resource was not found".to_string()))
}

#[get("/bad-request-test")]
async fn handle_bad_request(_req: Req) -> AppResult<Res> {
    Err(AppError::BadRequest("Invalid query parameters provided".to_string()))
}

#[get("/internal-test")]
async fn handle_internal(_req: Req) -> AppResult<Res> {
    Err(AppError::Internal("Something broke internally".to_string()))
}

#[tokio::test]
async fn test_error_propagation_and_response() {
    let mut app = App::new();

    // Автоматически регистрируем все роуты, помеченные макросами в этом тесте
    app.register_collected().unwrap();

    tokio::spawn(async move {
        let _ = app.listen("127.0.0.1:8083").await;
    });

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let client = reqwest::Client::new();

    let resp = client.get("http://127.0.0.1:8083/not-found-test").send().await.unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::NOT_FOUND);
    let body = resp.text().await.unwrap();
    assert!(body.contains("Custom resource was not found"));

    let resp = client.get("http://127.0.0.1:8083/bad-request-test").send().await.unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::BAD_REQUEST);
    let body = resp.text().await.unwrap();
    assert!(body.contains("Invalid query parameters provided"));

    let resp = client.get("http://127.0.0.1:8083/internal-test").send().await.unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::INTERNAL_SERVER_ERROR);
    let body = resp.text().await.unwrap();
    assert!(body.contains("Something broke internally"));

    println!("Все тесты обработки ошибок прошли успешно!");
}