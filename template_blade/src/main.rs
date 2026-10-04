mod config;
mod models;
mod repositories;
mod services;
mod controllers;
mod routes; 

use config::Config;
use really_fast_api::{App, Migrator};
use routes::{web, api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::init();
    let db_pool = cfg.connect_db().await;

    Migrator::run(&db_pool).await?;

    let mut app = App::new();
    app.manage(db_pool);

    app.register_collected().unwrap();

    println!("🚀 ReallyFastAPI server running at http://127.0.0.1:8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}