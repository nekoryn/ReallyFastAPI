use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    OK = 200,
    Created = 201,
    NoContent = 204,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    Conflict = 409,
    UnprocessableEntity = 422,
    InternalServerError = 500,
}

impl StatusCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatusCode::OK => "200 OK",
            StatusCode::Created => "201 Created",
            StatusCode::NoContent => "204 No Content",
            StatusCode::BadRequest => "400 Bad Request",
            StatusCode::Unauthorized => "401 Unauthorized",
            StatusCode::Forbidden => "403 Forbidden",
            StatusCode::NotFound => "404 Not Found",
            StatusCode::Conflict => "409 Conflict",
            StatusCode::UnprocessableEntity => "422 Unprocessable Entity",
            StatusCode::InternalServerError => "500 Internal Server Error",
        }
    }
}

pub struct Res {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Res {
    fn with_text_content_type(mut self) -> Self {
        self.headers.insert("Content-Type".to_string(), "text/plain; charset=utf-8".to_string());
        self
    }

    pub fn header(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.headers.insert(key.into(), val.into());
        self
    }

    pub fn ok_200(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::OK,
            headers: HashMap::new(),
            body: body.into(),
        }.with_text_content_type()
    }

    pub fn created_201(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::Created,
            headers: HashMap::new(),
            body: body.into(),
        }.with_text_content_type()
    }

    pub fn no_content_204(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NoContent,
            headers: HashMap::new(),
            body: body.into(),
        }.with_text_content_type()
    }

    pub fn bad_request_400(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BadRequest,
            headers: HashMap::new(),
            body: body.into(),
        }.with_text_content_type()
    }

    pub fn unauthorized_401(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::Unauthorized,
            headers: HashMap::new(),
            body: body.into(),
        }.with_text_content_type()
    }

    pub fn forbidden_403(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::Forbidden,
            headers: HashMap::new(),
            body: body.into(),
        }.with_text_content_type()
    }

    pub fn not_found_404(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NotFound,
            headers: HashMap::new(),
            body: body.into(),
        }.with_text_content_type()
    }

    pub fn conflict_409(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::Conflict,
            headers: HashMap::new(),
            body: body.into(),
        }.with_text_content_type()
    }

    pub fn unprocessable_entity_422(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UnprocessableEntity,
            headers: HashMap::new(),
            body: body.into(),
        }.with_text_content_type()
    }

    pub fn internal_server_error_500(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::InternalServerError,
            headers: HashMap::new(),
            body: body.into(),
        }.with_text_content_type()
    }

    pub fn html(body: impl Into<String>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());
        
        Self {
            status: StatusCode::OK,
            headers,
            body: body.into(),
        }
    }

    pub fn text_404(body: impl Into<String>) -> Self {
        Self::not_found_404(body)
    }

    pub fn file(file_path: &std::path::Path) -> Self {
        if !file_path.exists() || !file_path.is_file() {
            return Self::not_found_404("File not found");
        }

        let extension = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let mime_type = match extension {
            "html" => "text/html; charset=utf-8",
            "css" => "text/css; charset=utf-8",
            "js" => "application/javascript; charset=utf-8",
            "svg" => "image/svg+xml",
            _ => "text/plain; charset=utf-8",
        };

        match std::fs::read_to_string(file_path) {
            Ok(content) => {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), mime_type.to_string());
                Self {
                    status: StatusCode::OK,
                    headers,
                    body: content,
                }
            }
            Err(_) => Self::internal_server_error_500("Error reading file"),
        }
    }

    pub fn json<T: Serialize>(data: &T) -> Self {
        match serde_json::to_string(data) {
            Ok(json_str) => {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());

                Self {
                    status: StatusCode::OK,
                    headers,
                    body: json_str,
                }
            }

            Err(_) => Self::internal_server_error_500("Failed to serialize JSON")
        }
    }
}