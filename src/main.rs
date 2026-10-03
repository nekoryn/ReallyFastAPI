use really_fast_api::{App, Res};
use serde::{Serialize, Deserialize};

mod logger_middleware;
use logger_middleware::LoggerMiddleware;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct GreetParams {
    name: Option<String>,
    age: Option<u32>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    app.add_middleware(LoggerMiddleware);

    // Главная страница
    app.get("/", async |_req| {
        Res::ok_200("Hello from modular framework!")
    });

    // Получение JSON-пользователя
    app.get("/user", async |_req| {
        let user = User {
            name: "Nektaryn".to_string(),
            age: 18,
        };
        Res::json(&user)
    });

    // Создание пользователя через POST с парсингом JSON
    app.post("/user", async |req| {
        match req.json::<User>() {
            Ok(user) => {
                println!("Получен юзер через POST: {:?}", user);
                Res::json(&user)
            }
            Err(_) => Res::bad_request_400("Invalid JSON"),
        }
    });

    // PUT-запрос для обновления
    app.put("/update", async |req| {
        Res::ok_200(format!("Updated with: {}", req.body))
    });

    // DELETE-запрос для удаления
    app.delete("/remove", async |_req| {
        Res::ok_200("Deleted!")
    });

    // Динамический роут с параметром (в matchit 0.9 синтаксис через {})
    app.get("/users/{id}", async |req| {
        let binding = "unknown".to_string();
        let user_id = req.params.get("id").unwrap_or(&binding);
        Res::ok_200(format!("Fetching profile for user_id: {}", user_id))
    });

    app.get("/greet", async |req| {
        let name = req.query("name").unwrap_or_else(|| "Гость".to_string());
        Res::ok_200(format!("Привет, {}!", name))
    });

    app.get("/search", async |req| {
        let params: GreetParams = req.query_as().unwrap_or(GreetParams {
            name: None,
            age: None,
        });

        Res::json(&params)
    });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}