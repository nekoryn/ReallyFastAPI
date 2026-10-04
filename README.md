## Server
Сетевой слой фреймворка. Отвечает за запуск HTTP-сервера, прием входящих соединений и автоматическую поддержку протоколов **HTTP/1.1** и **HTTP/2** «из коробки».
### Usage
```rust
use really_fast_api::{App, Res, Req};
use really_fast_api::error::AppResult;
use really_fast_api::get;

#[get("/")]
async fn index(_req: Req) -> AppResult<Res> {
    Ok(Res::ok_200("Hello, World!"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    // Автоматически собираем и регистрируем все роуты, помеченные макросами
    app.register_collected().unwrap();

    // Запускаем сервер на нужном адресе и порту
    println!("Сервер запущен!");
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
```
## Router and App core
Ядро маршрутизации фреймворка. Основано на сверхбыстром крейте `matchit` (обеспечивает $O(1)$-производительность) и поддерживает декларативные процедурные макросы (`#[get]`, `#[post]`, `#[put]`, `#[delete]`), автоматическую сборку через `inventory`, динамические параметры в URL, глобальное состояние и локальные middleware
### Usage
```rust
use really_fast_api::{App, Res, Req};
use really_fast_api::error::AppResult;
use really_fast_api::{get, post, put, delete};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct UserDto {
    name: String,
    age: u8,
}

// 1. Простой GET-запрос через макрос
#[get("/")]
async fn handle_root(_req: Req) -> AppResult<Res> {
    Ok(Res::ok_200("Welcome home!"))
}

// 2. GET-запрос с динамическим параметром в URL
#[get("/users/{id}")]
async fn handle_get_user(req: Req) -> AppResult<Res> {
    let default_val = "unknown".to_string();
    let user_id = req.params.get("id").unwrap_or(&default_val);
    Ok(Res::ok_200(format!("Profile ID: {}", user_id)))
}

// 3. POST-запрос с разбором JSON тела
#[post("/users")]
async fn handle_create_user(req: Req) -> AppResult<Res> {
    match req.json::<UserDto>() {
        Ok(user) => Ok(Res::created_201(format!("User {} created successfully!", user.name))),
        Err(_) => Ok(Res::bad_request_400("Invalid JSON format")),
    }
}

// 4. PUT-запрос
#[put("/users/{id}")]
async fn handle_update_user(req: Req) -> AppResult<Res> {
    let id = req.params.get("id").unwrap_or(&"1".to_string());
    Ok(Res::ok_200(format!("User {} updated with body: {}", id, req.body)))
}

// 5. DELETE-запрос
#[delete("/users/{id}")]
async fn handle_delete_user(req: Req) -> AppResult<Res> {
    let id = req.params.get("id").unwrap_or(&"1".to_string());
    Ok(Res::ok_200(format!("User {} deleted", id)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    // Автоматическая регистрация всех роутов, помеченных макросами
    app.register_collected().unwrap();

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
```
## Response
Внутри хендлеров роутов вы можете использовать готовые шорткаты для формирования ответа.
### Текстовые ответы и статусы
Каждый шорткат автоматически проставляет заголовок `Content-Type: text/plain; charset=utf-8`:

```rust
app.get("/hello", async |_req| {
    Res::ok_200("Hello, World!")
});

app.post("/items", async |_req| {
    Res::created_201("Item successfully created")
});

app.get("/missing", async |_req| {
    Res::not_found_404("Resource not found")
});
```
### Возврат JSON-данных
Метод `Res::json(&data)` автоматически сериализует любую структуру, реализующую `serde::Serialize`, превращает её в строку и устанавливает заголовок `Content-Type: application/json`:

```rust
use serde::Serialize;

#[derive(Serialize)]
struct UserResponse {
    id: u64,
    name: String,
}

app.get("/user/profile", async |_req| {
    let user = UserResponse {
        id: 42,
        name: "Alex".to_string(),
    };
    
    Res::json(&user)
});
```
### Добавление кастомных заголовков
Все методы ответа поддерживают цепочку вызовов (`builder pattern`) через `.header(key, val)`, что позволяет гибко добавлять кастомные заголовки (например, для CORS, токенов авторизации или кеширования):

```rust
app.get("/custom-header", async |_req| {
    Res::ok_200("Check headers")
        .header("X-Custom-Token", "secret-123")
        .header("Cache-Control", "no-store")
});
```
### Поддерживаемые статус-коды
Модуль содержит встроенный перечисление (`StatusCode`), покрывающее основные потребности REST API:

- `StatusCode::OK` (200)
    
- `StatusCode::Created` (201)
    
- `StatusCode::NoContent` (204)
    
- `StatusCode::BadRequest` (400)
    
- `StatusCode::Unauthorized` (401)
    
- `StatusCode::Forbidden` (403)
    
- `StatusCode::NotFound` (404)
    
- `StatusCode::Conflict` (409)
    
- `StatusCode::UnprocessableEntity` (422)
    
- `StatusCode::InternalServerError` (500)
## Request
Модуль инкапсулирует всю информацию о входящем HTTP-запросе. Предоставляет удобный API для десериализации JSON-тела, извлечения динамических параметров роута, чтения query-параметров строки и доступа к внедренным зависимостям (Dependency Injection).
### Usage
Внутри хендлера или контроллера объект `Req` доступен в качестве аргумента.
#### Получение данных из тела запроса (JSON)
Метод `req.json::<T>()` автоматически парсит строку `body` в любую структуру, реализующую `serde::Deserialize`:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUserDto {
    name: String,
    age: u8,
}

app.post("/users", async |req| {
    match req.json::<CreateUserDto>() {
        Ok(dto) => Res::ok_200(format!("Created user: {}, age: {}", dto.name, dto.age)),
        Err(_) => Res::bad_request_400("Invalid JSON body"),
    }
});
```
#### Чтение Query-параметров
Вы можете получать отдельные параметры через `req.query("key")` или маппить всю строку целиком в структуру с помощью `req.query_as::<T>()`:
```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchParams {
    name: Option<String>,
    page: Option<u32>,
}

app.get("/search", async |req| {
    // Вариант А: точечное чтение параметра
    let name = req.query("name").unwrap_or_else(|| "Guest".to_string());

    // Вариант Б: парсинг всей query-строки в структуру
    let params: SearchParams = req.query_as().unwrap_or(SearchParams {
        name: None,
        page: None,
    });

    Res::ok_200(format!("Searching for: {}", name))
});
```
#### Доступ к глобальным зависимостям
Если вы сохранили пул базы данных или конфиг в приложение через `.manage(...)`, вы можете достать его в любом роуте с помощью `req.get::<T>()`:

```rust
use sqlx::PgPool;

app.get("/stats", async |req| {
    // Достаем пул соединений с БД, сохраненный через app.manage(db_pool)
    let db = req.get::<PgPool>().expect("DB pool not found!");

    // Выполняем логику...
    Res::ok_200("Database pool successfully accessed!")
});
```
### Структура
Помимо хелперов, структура напрямую предоставляет доступ к базовым полям:

- `req.method` (`String`) — HTTP-метод (`GET`, `POST`, и т.д.).
    
- `req.path` (`String`) — путь запроса без query-строки.
    
- `req.uri` (`hyper::Uri`) — полный URI объекта.
    
- `req.body` (`String`) — сырое тело запроса в виде строки.
    
- `req.params` (`HashMap<String, String>`) — динамические параметры пути (например, `{id}`).
    
- `req.extensions` (`http::Extensions`) — хранилище расширений Hyper, используемое для проброса зависимостей и стейта.
## Middleware
Модуль определяет систему промежуточного ПО (Middleware), которая позволяет перехватывать входящие запросы до того, как они попадут в контроллер, а также обрабатывать исходящие ответы.
### Usage
Чтобы создать свой собственный мидлвар (например, для логирования времени запроса или проверки токена авторизации), нужно реализовать трейт `Middleware`.
#### Написание кастомного Middleware
Каждый мидлвар принимает запрос `Req` и замыкание `next`, которое вызывает следующий шаг в цепочке (другой мидлвар или конечный контроллер):

```rust
use really_fast_api::{Req, Res};
use really_fast_api::middleware::{Middleware, Next, BoxFuture};

pub struct LoggerMiddleware;

impl Middleware for LoggerMiddleware {
    fn handle(&self, req: Req, next: Next<'static>) -> BoxFuture<'static, Res> {
        Box::pin(async move {
            let path = req.path.clone();
            println!("[LOG] Incoming request to: {}", path);

            // Передаем управление дальше по цепочке
            let res = next(req).await;

            println!("[LOG] Request to {} completed with status: {:?}", path, res.status);
            res
        })
    }
}
```
#### Регистрация Middleware
Мидлвары можно подключать двумя способами:

- **Глобально (на всё приложение):** Выполняется для абсолютно каждого входящего запроса.

```rust
let mut app = App::new();
app.add_middleware(LoggerMiddleware);
```
 
- **Локально (на конкретный роут):** Благодаря флюент-интерфейсу (`RouteBuilder`), мидлвар можно повесить точечно на конкретный эндпоинт (например, проверку прав доступа только для административных маршрутов):
```rust
app.get("/admin/dashboard", async |_req| {
    Res::ok_200("Welcome to admin panel")
}).middleware(AuthMiddleware);
```
#### Встроенный CORS Middleware
Фреймворк из коробки содержит готовый `CorsMiddleware` для настройки заголовков кросс-доменных запросов:

```rust
use really_fast_api::CorsMiddleware;

let mut app = App::new();
app.add_middleware(CorsMiddleware::new("http://localhost:3000"));
```

## CLI Tool
Инструмент командной строки фреймворка. Автоматизирует создание новых проектов, генерацию правильной структуры директорий (`MVC`-подобный каркас), настройку переменных окружения и автоматическое связывание с локальным исходным кодом фреймворка.
### Usage
#### Создание нового проекта
Для генерации нового сервиса используется команда `new` с указанием имени проекта:

```bash
$ really_fast_api new my_awesome_app
```

В результате выполнения утилита:

1. Создаст директорию проекта с готовой структурой папок:
    
    - `src/controllers/` — для логики обработки запросов.
        
    - `src/repositories/` — для работы с базой данных.
        
    - `src/models/` — для структур данных и DTO.
        
2. Сгенерирует файл `.env` с дефолтными настройками подключения к PostgreSQL.
    
3. Сформирует современный `Cargo.toml` со всеми необходимыми зависимостями (`tokio`, `hyper`, `sqlx`, `serde`, `dotenvy`) и пропишет абсолютный путь к твоему локальному фреймворку через `path`.
    
4. Создаст готовый конфигурационный файл `src/config.rs` для работы с переменными окружения и пулом соединений.
    
5. Напишет стартовый шаблон `src/main.rs` с уже внедренным в стейт (`App::manage`) пулом базы данных.
#### Сгенерированная структура
Проект сразу готов к запуску и работе с базой данных без ручной настройки бойлерплейта:
```rust 
mod config;
use config::Config;
use really_fast_api::{App, Res};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::init();
    let db_pool = cfg.connect_db().await;

    let mut app = App::new();
    app.manage(db_pool); // Пул БД сразу доступен в контроллерах через req.get::<PgPool>()

    app.get("/", async |_req| {
        Res::ok_200("Hello from ReallyFastAPI!")
    });

    println!("ReallyFastAPI server running at http://127.0.0.1:8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
```
### Что делает CLI под капотом
- **Динамический `Cargo.toml`:** Использует макрос компиляции `env!("CARGO_MANIFEST_DIR")`, чтобы при сборке CLI автоматически подставлять реальный абсолютный путь к твоему фреймворку на машине разработчика. Это позволяет моментально тестировать локальные изменения в движке без публикации на crates.io.
    
- **Безопасность:** Проверяет существование папки перед созданием, исключая случайную перезапись уже существующих проектов.
## Обработка ошибок
Хендлеры возвращают `AppResult<T> = Result<T, AppError>`. Фреймворк автоматически перехватывает ошибки и преобразует их в соответствующие HTTP-ответы:

```rust
use really_fast_api::error::{AppError, AppResult};

#[get("/error-example")]
async fn trigger_error(_req: Req) -> AppResult<Res> {
    // Автоматически вернет статус 404 Not Found
    Err(AppError::NotFound("Resource does not exist".to_string()))
}
```
_Поддерживаемые варианты ошибок:_ `AppError::Database`, `AppError::Json`, `AppError::Validation`, `AppError::Unauthorized`, `AppError::Forbidden`, `AppError::NotFound`, `AppError::BadRequest`, `AppError::Internal`.