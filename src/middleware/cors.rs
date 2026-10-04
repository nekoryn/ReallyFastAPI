use crate::{Req, Res};
use crate::middleware::{Middleware, Next, BoxFuture};
use crate::error::AppResult;

pub struct CorsMiddleware {
    allowed_origin: String,
}

impl CorsMiddleware {
    pub fn new(allowed_origin: impl Into<String>) -> Self {
        Self {
            allowed_origin: allowed_origin.into(),
        }
    }
}

impl Middleware for CorsMiddleware {
    fn handle(&self, req: Req, next: Next<'static, AppResult<Res>>) -> BoxFuture<'static, AppResult<Res>> {
        let origin = self.allowed_origin.clone();
        
        Box::pin(async move {
            if req.method == "OPTIONS" {
                let mut res = Res::ok_200("");
                res.headers.insert("Access-Control-Allow-Origin".to_string(), origin);
                res.headers.insert("Access-Control-Allow-Methods".to_string(), "GET, POST, PUT, DELETE, OPTIONS".to_string());
                res.headers.insert("Access-Control-Allow-Headers".to_string(), "Content-Type, Authorization".to_string());
                return Ok(res);
            }

            // Передаем управление дальше по цепочке
            let result = next(req).await;

            // Если выполнение прошло успешно, добавляем CORS-заголовки к ответу
            match result {
                Ok(mut res) => {
                    res.headers.insert("Access-Control-Allow-Origin".to_string(), origin);
                    res.headers.insert("Access-Control-Allow-Methods".to_string(), "GET, POST, PUT, DELETE, OPTIONS".to_string());
                    res.headers.insert("Access-Control-Allow-Headers".to_string(), "Content-Type, Authorization".to_string());
                    Ok(res)
                }
                // Если дальше по цепочке произошла ошибка, её поймает наш обработчик ошибок
                Err(err) => Err(err),
            }
        })
    }
}