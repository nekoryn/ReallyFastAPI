use really_fast_api::router::App;
use really_fast_api::response::Res;
use really_fast_api::request::Req;
use really_fast_api::middleware::{Middleware, Next, BoxFuture};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

struct GlobalTestMiddleware {
    executed: Arc<AtomicBool>,
}

impl Middleware for GlobalTestMiddleware {
    fn handle(&self, req: Req, next: Next<'static>) -> BoxFuture<'static, Res> {
        self.executed.store(true, Ordering::SeqCst);

        Box::pin(async move {
            let res = next(req).await;
            let mut r = res;
            r.headers.insert("X-Global-Middleware".to_string(), "active".to_string());
            r
        })
    }
}

struct LocalTestMiddleware;

impl Middleware for LocalTestMiddleware {
    fn handle(&self, req: Req, next: Next<'static>) -> BoxFuture<'static, Res> {
        Box::pin(async move {
            let res = next(req).await;
            let mut r = res;
            r.headers.insert("X-Local-Middleware".to_string(), "active".to_string());
            r
        })
    }
}

#[tokio::test]
async fn test_global_and_local_middlewares() {
    let global_flag = Arc::new(AtomicBool::new(false));

    let mut app = App::new();

    app.add_middleware(GlobalTestMiddleware {
        executed: global_flag.clone(),
    });

    app.get("/global-only", async |_req| {
        Res::ok_200("Global only route")
    });

    app.get("/protected", async |_req| {
        Res::ok_200("Protected route content")
    }).middleware(LocalTestMiddleware);

    let matched_global = app.get_router.at("/global-only");
    assert!(matched_global.is_ok(), "Маршрут /global-only должен находится");

    let matched_protected = app.get_router.at("/protected");
    assert!(matched_protected.is_ok(), "Маршрут /protected должен находится");

    assert_eq!(matched_protected.unwrap().value.middlewares.len(), 1, "У /protected должен быть ровно 1 локальный мидлвеер");

    assert_eq!(matched_global.unwrap().value.middlewares.len(), 0, "У /global-only должно быть 0 локальных мидлвееров");
}