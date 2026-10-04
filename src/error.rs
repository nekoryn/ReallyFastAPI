use crate::response::Res;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    Json(serde_json::Error),
    Validation(String),
    Unauthorized(String),
    Forbidden(String),
    RouteConflict(String),
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        eprintln!("[DB ERROR]: {:?}", err);
        AppError::Database(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Json(err)
    }
}

impl AppError {
    pub fn into_response(&self) -> Res {
        match self {
            AppError::Database(_) => {
                Res::internal_server_error_500("Internal Database Error")
            }
            AppError::Json(err) => {
                Res::bad_request_400(format!("Invalid JSON format: {}", err))
            }
            AppError::Validation(msg) => {
                Res::unprocessable_entity_422(format!("Validation error: {}", msg))
            }
            AppError::Unauthorized(msg) => {
                Res::unauthorized_401(msg.clone())
            }
            AppError::Forbidden(msg) => {
                Res::forbidden_403(msg.clone())
            }
            AppError::RouteConflict(msg) => {
                Res::internal_server_error_500(format!("Route Conflict Error: {}", msg))
            }
            AppError::NotFound(msg) => {
                Res::not_found_404(msg.clone())
            }
            AppError::BadRequest(msg) => {
                Res::bad_request_400(msg.clone())
            }
            AppError::Internal(msg) => {
                Res::internal_server_error_500(msg.clone())
            }
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;