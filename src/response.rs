use std::{collections::HashMap};
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
    pub fn ok_200(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::OK,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    pub fn created_201(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::Created,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    pub fn no_content_204(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NoContent,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    pub fn bad_request_400(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BadRequest,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    pub fn unauthorized_401(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::Unauthorized,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    pub fn forbidden_403(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::Forbidden,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    pub fn not_found_404(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NotFound,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    pub fn conflict_409(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::Conflict,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    pub fn unprocessable_entity_422(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UnprocessableEntity,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    pub fn internal_server_error_500(body: impl Into<String>) -> Self {
        Self {
            status: StatusCode::InternalServerError,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let res_txt = format!(
            "HTTP/1.1 {}\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", 
            self.status.as_str(),
            self.body.len(),
            self.body
        );
        res_txt.into_bytes()
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