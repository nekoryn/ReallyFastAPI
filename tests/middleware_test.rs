use really_fast_api::error::AppResult;
use really_fast_api::router::App;
use really_fast_api::response::Res;
use really_fast_api::request::Req;
use really_fast_api::middleware::{Middleware, Next, BoxFuture};
use really_fast_api::get;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

struct GlobalTestMiddleware {
    executed: Arc<AtomicBool>,
}

impl Middleware for GlobalTestMiddleware {
    fn handle(&self, req: Req, next: Next<'static, AppResult<Res>>) -> BoxFuture<'static, AppResult<Res>> {
        self.executed.store(true, Ordering::SeqCst);
        Box::pin(async move {
            let res = next(req).await;
            match res {
                Ok(mut r) => {
                    r.headers.insert("X-Global-Middleware".to_string(), "active".to_string());
                    Ok(r)
                }
                Err(e) => Err(e),
            }
        })
    }
}

#[get("/global-only")]
async fn handle_global_only(_req: Req) -> AppResult<Res> {
    Ok(Res::ok_200("Global only route"))
}

#[get("/protected")]
async fn handle_protected(_req: Req) -> AppResult<Res> {
    Ok(Res::ok_200("Protected route content"))
}

#[tokio::test]
async fn test_global_and_local_middlewares() {
    let global_flag = Arc::new(AtomicBool::new(false));

    let mut app = App::new();

    app.add_middleware(GlobalTestMiddleware {
        executed: global_flag.clone(),
    });

    app.register_collected().unwrap();

    let matched_global = app.get_router.at("/global-only");
    assert!(matched_global.is_ok(), "Маршрут /global-only должен находиться");

    let matched_protected = app.get_router.at("/protected");
    assert!(matched_protected.is_ok(), "Маршрут /protected должен находиться");

    assert_eq!(matched_global.unwrap().value.middlewares.len(), 0, "У /global-only должно быть 0 локальных мидлвееров");
}