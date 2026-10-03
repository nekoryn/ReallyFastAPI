use std::fs;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[1] != "new" {
        println!("Usage: really_fast_api new <project_name>");
        return;
    }

    let project_name = &args[2];
    let project_path = Path::new(project_name);

    if project_path.exists() {
        eprintln!("Error: Directory {} already exists!", project_name);
        return;
    }

    println!("Creating new ReallyFastAPI project: {}...", project_name);

    // 1. Создаем структуру папок внутри src
    fs::create_dir_all(project_path.join("src/controllers")).unwrap();
    fs::create_dir_all(project_path.join("src/repositories")).unwrap();
    fs::create_dir_all(project_path.join("src/models")).unwrap();

    // 2. Создаем .env
    let env_content = "DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres\n";
    fs::write(project_path.join(".env"), env_content).unwrap();

    // Получаем абсолютный путь к текущему фреймворку при компиляции CLI
    let framework_path = env!("CARGO_MANIFEST_DIR");

    // 3. Генерируем Cargo.toml с правильной локальной зависимостью
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
        
[dependencies]
tokio = {{ version = "1", features = ["full"] }}
hyper = {{ version = "1", features = ["full"] }}
hyper-util = {{ version = "0.1", features = ["full"] }}
http-body-util = "0.1"
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
matchit = "0.9.2"
serde_urlencoded = "0.7"
sqlx = {{ version = "0.9", features = ["runtime-tokio", "postgres", "macros", "chrono"] }}
dotenvy = "0.15"
really_fast_api = {{ path = "{}" }}

[dev-dependencies]
reqwest = {{ version = "0.13", features = ["json"] }}
"#,
        project_name, framework_path
    );
    fs::write(project_path.join("Cargo.toml"), cargo_toml).unwrap();

    // 4. Пишем конфиг прямо в src/config.rs, чтобы mod config; работал идеально
    let config_code = r#"
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
"#;
    fs::write(project_path.join("src/config.rs"), config_code.trim()).unwrap();

    // 5. Генерируем src/main.rs
    let main_code = r#"
mod config;
use config::Config;
use really_fast_api::{App, Res};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::init();
    let db_pool = cfg.connect_db().await;

    let mut app = App::new();
    app.manage(db_pool);

    app.get("/", async |_req| {
        Res::ok_200("Hello from ReallyFastAPI!")
    });

    println!("ReallyFastAPI server running at http://127.0.0.1:8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
"#;
    fs::write(project_path.join("src/main.rs"), main_code.trim()).unwrap();

    println!("Project '{}' successfully created!", project_name);
}