// --- Внутренние модули фреймворка ---
pub mod error;
pub mod request;
pub mod response;
pub mod router;
pub mod server;
pub mod middleware;
pub mod migration;
pub mod models;
pub mod config;
pub mod view;

// --- Удобные реэкспорты (Public API) ---
pub use error::{AppError, AppResult};
pub use request::Req;
pub use response::{Res, StatusCode};
pub use router::App;
pub use middleware::CorsMiddleware;
pub use migration::Migrator;
pub use config::Config;
pub use view::{ViewEngine, BladeEngine};

// --- Процедурные макросы роутинга ---
pub use really_fast_api_macros::{get, post, put, delete};