use really_fast_api::get;
use really_fast_api::{Res, Req};
use really_fast_api::error::AppResult;
use really_fast_api::view::{BladeEngine, ViewEngine};

#[get("/")]
pub async fn welcome(_req: Req) -> AppResult<Res> {
    let mut data = std::collections::HashMap::new();
    data.insert("title".to_string(), "Welcome to ReallyFastAPI".to_string());

    let html = BladeEngine::new("resources/views")
        .render("welcome", data)
        .unwrap_or_else(|_| "View not found".to_string());

    Ok(Res::html(html))
}

#[get("/css/app.css")]
pub async fn serve_css(_req: Req) -> AppResult<Res> {
    let path = std::path::Path::new("public/css/app.css");
    Ok(Res::file(path))
}


#[get("/js/app.js")]
pub async fn serve_js(_req: Req) -> AppResult<Res> {
    Ok(Res::file(std::path::Path::new("public/js/app.js")))
}