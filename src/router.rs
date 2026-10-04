use matchit::Router;
use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;
use hyper::http::Extensions;

use crate::request::Req;
use crate::response::Res;
use crate::middleware::{Middleware};
use crate::error::{AppError, AppResult};

pub type RouteHandler = Arc<dyn Fn(Req) -> Pin<Box<dyn Future<Output =AppResult<Res>> + Send>> + Send + Sync>;

pub struct RouteEntry {
    pub handler: RouteHandler,
    pub middlewares: Vec<Arc<dyn Middleware>>,
}

pub struct RouteBuilder<'a> {
    entry: &'a mut RouteEntry,
}

impl<'a> RouteBuilder<'a> {
    pub fn middleware<M: Middleware + 'static>(&mut self, middleware: M) -> &mut Self {
        self.entry.middlewares.push(Arc::new(middleware));
        self
    }
}

pub struct App {
    pub get_router: Router<RouteEntry>,
    pub post_router: Router<RouteEntry>,
    pub put_router: Router<RouteEntry>,
    pub delete_router: Router<RouteEntry>,
    pub middlewares: Vec<Arc<dyn Middleware>>,
    pub extensions: Extensions,
}

impl App {
    pub fn new() -> Self {
        Self {
            get_router: Router::new(),
            post_router: Router::new(),
            put_router: Router::new(),
            delete_router: Router::new(),
            middlewares: Vec::new(),
            extensions: Extensions::new(),
        }
    }

    pub fn manage<T: Clone + Send + Sync + 'static>(&mut self, value: T) -> &mut Self {
        self.extensions.insert(value);
        self
    }

    pub fn add_middleware<M: Middleware + 'static>(&mut self, middleware: M) {
        self.middlewares.push(Arc::new(middleware));
    }

    pub fn get<F, Fut>(&mut self, path: &str, handler: F) -> AppResult<RouteBuilder<'_>>
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = AppResult<Res>> + Send + 'static,
    {
        let entry = RouteEntry {
            handler: Arc::new(move |req| Box::pin(handler(req))),
            middlewares: Vec::new(),
        };
        self.get_router
            .insert(path.to_string(), entry)
            .map_err(|e| AppError::RouteConflict(format!("Duplicate or invalid GET route '{}': {}", path, e)))?;

        let inserted = self.get_router
            .at_mut(path)
            .map_err(|e| AppError::Internal(format!("Failed to retrieve inserted route '{}': {}", path, e)))?;

        Ok(RouteBuilder { entry: inserted.value })
    }

    pub fn post<F, Fut>(&mut self, path: &str, handler: F) -> AppResult<RouteBuilder<'_>>
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = AppResult<Res>> + Send + 'static,
    {
        let entry = RouteEntry {
            handler: Arc::new(move |req| Box::pin(handler(req))),
            middlewares: Vec::new(),
        };

        self.post_router
            .insert(path.to_string(), entry)
            .map_err(|e| AppError::RouteConflict(format!("Duplicate or invalid POST route '{}': {}", path, e)))?;

        let inserted = self.post_router
            .at_mut(path)
            .map_err(|e| AppError::Internal(format!("Failed to retrieve inserted route '{}': {}", path, e)))?;

        Ok(RouteBuilder { entry: inserted.value })
    }

    pub fn put<F, Fut>(&mut self, path: &str, handler: F) -> AppResult<RouteBuilder<'_>>
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = AppResult<Res>> + Send + 'static,
    {
        let entry = RouteEntry {
            handler: Arc::new(move |req| Box::pin(handler(req))),
            middlewares: Vec::new(),
        };

        self.put_router
            .insert(path.to_string(), entry)
            .map_err(|e| AppError::RouteConflict(format!("Duplicate or invalid PUT route '{}': {}", path, e)))?;

        let inserted = self.put_router
            .at_mut(path)
            .map_err(|e| AppError::RouteConflict(format!("Failed to retrieve inserted route '{}': {}", path, e)))?;

        Ok(RouteBuilder { entry: inserted.value })

    }

    pub fn delete<F, Fut>(&mut self, path: &str, handler: F) -> AppResult<RouteBuilder<'_>>
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = AppResult<Res>> + Send + 'static,
    {
        let entry = RouteEntry {
            handler: Arc::new(move |req| Box::pin(handler(req))),
            middlewares: Vec::new(),
        };

        self.delete_router
            .insert(path.to_string(), entry)
            .map_err(|e| AppError::RouteConflict(format!("Duplicate or invalid DELETE route '{}': {}", path, e)))?;

        let inserted = self.delete_router
            .at_mut(path)
            .map_err(|e| AppError::RouteConflict(format!("Failed to retrieve inserted route '{}': {}", path, e)))?;

        Ok(RouteBuilder { entry: inserted.value})
    }
}