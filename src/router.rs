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
    pub options_router: Router<RouteEntry>,
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
            options_router: Router::new(),
            middlewares: Vec::new(),
            extensions: Extensions::new(),
        }
    }

    fn register_route(
        router: &mut matchit::Router<RouteEntry>,
        options_router: &mut matchit::Router<RouteEntry>,
        path: &str,
        handler: RouteHandler,
        method_name: &str,
    ) -> AppResult<()> {
        let entry = RouteEntry {
            handler: handler.clone(),
            middlewares: Vec::new(),
        };

        router
            .insert(path.to_string(), entry)
            .map_err(|e| AppError::RouteConflict(format!("Duplicate or invalid {} route '{}': {}", method_name, path, e)))?;

        let options_entry = RouteEntry {
            handler: Arc::new(|_| Box::pin(async { Ok(Res::ok_200("")) })),
            middlewares: Vec::new(),
        };
        let _ = options_router.insert(path.to_string(), options_entry);

        Ok(())
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
        let route_handler: RouteHandler = Arc::new(move |req| Box::pin(handler(req)));

        Self::register_route(
            &mut self.get_router, 
            &mut self.options_router, 
            path, route_handler, 
            "GET"
        )?;

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
        let route_handler: RouteHandler = Arc::new(move |req| Box::pin(handler(req)));

        Self::register_route(
            &mut self.post_router, 
            &mut self.options_router, 
            path, route_handler, 
            "POST"
        )?;
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
        let route_handler: RouteHandler = Arc::new(move |req| Box::pin(handler(req)));

        Self::register_route(
            &mut self.put_router, 
            &mut self.options_router, 
            path, route_handler, 
            "PUT"
        )?;

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
        let route_handler: RouteHandler = Arc::new(move |req| Box::pin(handler(req)));

        Self::register_route(
            &mut self.delete_router, 
            &mut self.options_router, 
            path, route_handler, 
            "DELETE"
        )?;
        let inserted = self.delete_router
            .at_mut(path)
            .map_err(|e| AppError::RouteConflict(format!("Failed to retrieve inserted route '{}': {}", path, e)))?;

        Ok(RouteBuilder { entry: inserted.value})
    }

    pub fn options<F, Fut>(&mut self, path: &str, handler: F) -> AppResult<RouteBuilder<'_>>
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = AppResult<Res>> + Send + 'static,
    {
        let entry = RouteEntry {
            handler: Arc::new(move |req| Box::pin(handler(req))),
            middlewares: Vec::new(),
        };

        self.options_router
            .insert(path.to_string(), entry)
            .map_err(|e| AppError::RouteConflict(format!("Duplicate or invalid OPTIONS route '{}': {}", path, e)))?;

        let inserted = self.options_router
            .at_mut(path)
            .map_err(|e| AppError::RouteConflict(format!("Failed to retrieve inserted route '{}': {}", path, e)))?;

        Ok(RouteBuilder { entry: inserted.value })
    }
}

pub struct RouteRegistration {
    pub method: &'static str,
    pub path: &'static str,
    pub handler_fn: fn() -> RouteHandler,
}

inventory::collect!(RouteRegistration);

impl App {
    /// Единый метод для автоматической регистрации всех роутов, помеченных макросами
    pub fn register_collected(&mut self) -> AppResult<()> {
        for reg in inventory::iter::<RouteRegistration>() {
            let (router, method_name) = match reg.method {
                "GET" => (&mut self.get_router, "GET"),
                "POST" => (&mut self.post_router, "POST"),
                "PUT" => (&mut self.put_router, "PUT"),
                "DELETE" => (&mut self.delete_router, "DELETE"),
                _ => continue,
            };

            let handler = (reg.handler_fn)();

            Self::register_route(
                router,
                &mut self.options_router,
                reg.path,
                handler,
                method_name,
            )?;
        }
        Ok(())
    }
}