use std::future::Future;
use std::pin::Pin;

use crate::request::Req;
use crate::response::Res;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub type Next<'static_ft> = Box<dyn FnOnce(Req) -> BoxFuture<'static_ft, Res> + Send + 'static_ft>;

pub trait Middleware: Send + Sync {
    fn handle(&self, req: Req, next: Next<'static>) -> BoxFuture<'static, Res>;
}