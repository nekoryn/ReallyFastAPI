use really_fast_api::get;
use really_fast_api::{Res, Req};
use really_fast_api::error::AppResult;
use crate::controllers::user_controller::UserController;

#[get("/users")]
pub async fn get_users(req: Req) -> AppResult<Res> {
    // Вызываем метод из UserController, который мы писали ранее
    UserController::get_users(req).await
}