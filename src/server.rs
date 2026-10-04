use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;

use hyper_util::server::conn::auto;
use tokio::net::TcpListener;
use hyper::{Request, Response};
use hyper::body::Incoming;
use hyper_util::rt::{TokioExecutor, TokioIo};
use http_body_util::Full;
use http_body_util::BodyExt;
use hyper::body::Bytes;

use crate::router::App;
use crate::request::Req;
use crate::response::Res;

impl App {
    pub fn publish_assets(&self) {
    let public_css = std::path::Path::new("public/css");
    let public_js = std::path::Path::new("public/js");

    let _ = std::fs::create_dir_all(public_css);
    let _ = std::fs::create_dir_all(public_js);

    let target_css = public_css.join("app.css");
    let res_css = std::path::Path::new("resources/css/app.css");
    if res_css.exists() {
        let _ = std::fs::copy(res_css, &target_css);
    } else if !target_css.exists() {
        let default_css = "body { background-color: #0f172a; color: #f8fafc; font-family: system-ui, sans-serif; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; }";
        let _ = std::fs::write(&target_css, default_css);
    }

    let target_js = public_js.join("app.js");
    let res_js = std::path::Path::new("resources/js/app.js");
    if res_js.exists() {
        let _ = std::fs::copy(res_js, &target_js);
    } else {
        let default_js = "console.log('🚀 ReallyFastAPI JavaScript loaded successfully!');";
        let _ = std::fs::write(&target_js, default_js);
    }
}

    pub async fn listen(self, addr_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.publish_assets();
        let addr: SocketAddr = addr_str.parse()?;
        let listener = TcpListener::bind(addr).await?;
        println!("Success! Industrial server running at http://{}", addr);

        let app = std::sync::Arc::new(self);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let app_clone = app.clone();

            tokio::spawn(async move {
                let service = hyper::service::service_fn(move |req: Request<Incoming>| {
                    let app = app_clone.clone();
                    async move {
                        handle_hyper_request(app, req).await
                    }
                });

                let builder = auto::Builder::new(TokioExecutor::new());
                if let Err(err) = builder.serve_connection(io, service).await {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

async fn handle_hyper_request(
    app: std::sync::Arc<App>,
    req: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let method = req.method().to_string();
    let uri = req.uri().clone();
    let path = req.uri().path().to_string();

    let whole_body = req.collect().await.map(|b| b.to_bytes()).unwrap_or_default();
    let body_str = String::from_utf8_lossy(&whole_body).to_string();

    let mut params = HashMap::new();

    let handler_result = match method.as_str() {
        "GET" => app.get_router.at(&path),
        "POST" => app.post_router.at(&path),
        "PUT" => app.put_router.at(&path),
        "DELETE" => app.delete_router.at(&path),
        "OPTIONS" => app.options_router.at(&path),
        _ => Err(matchit::MatchError::NotFound),
    };

    let custom_res = match handler_result {
        Ok(matched) => {
            for (key, val) in matched.params.iter() {
                params.insert(key.to_string(), val.to_string());
            }

            let custom_req = Req {
                method: method.clone(),
                path: path.clone(),
                uri: uri.clone(),
                body: body_str,
                params,
                extensions: app.extensions.clone(),
            };
            
            let route_entry = matched.value;

            let handler = route_entry.handler.clone();

            let mut next: crate::middleware::Next<'static, crate::error::AppResult<Res>> = Box::new(move |req|{
                let h = handler.clone();
                Box::pin(async move {
                    (h)(req).await
                })
            });
            
            let mut all_middlewares = app.middlewares.clone();
            all_middlewares.extend(route_entry.middlewares.clone());

            for mw in all_middlewares.iter().rev() {
                let mw_clone = mw.clone();
                let current_next = next;

                next = Box::new(move |req|{
                    mw_clone.handle(req, current_next)
                });
            }
            match next(custom_req).await {
                Ok(res) => res,
                Err(err) => err.into_response(),
            }
        }
        Err(_) => Res::not_found_404("404 Not Found... :("),
    };

    let status_code_u16 = custom_res.status as u16;
    let mut builder = Response::builder().status(status_code_u16);

    for (key, val) in custom_res.headers {
        builder = builder.header(key, val);
    }

    let response = match builder.body(Full::new(Bytes::from(custom_res.body))) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Failed to build HTTP response: {}", err);
            Response::builder()
                .status(500)
                .body(Full::new(Bytes::from("Internal Server Error")))
                .unwrap()
        }
    };

    Ok(response) // Исправлено Ok вместо OK
}