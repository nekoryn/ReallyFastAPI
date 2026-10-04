use std::time::Instant;

use really_fast_api::middleware::{Middleware, Next, BoxFuture};
use really_fast_api::response::Res;
use really_fast_api::request::Req;

pub struct LoggerMiddleware;

impl Middleware for LoggerMiddleware {
    fn handle(&self, req: Req, next: Next<'static>) -> BoxFuture<'static, Res> {
        Box::pin(async move {
            let start = Instant::now();
            let method = req.method.clone();
            let path = req.path.clone();

            println!("-> [Incoming] {} {}", method, path);

            let res = next(req).await;

            let duration = start.elapsed();
            let status = res.status as u16;
            println!("<- [Finished] {} {} -> Status: {} (took {}ms)", method, path, status, duration.as_millis());

            res
        })
    }
}