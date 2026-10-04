use really_fast_api::{Res, Req};
use really_fast_api::error::AppResult;
use crate::services::user_service::UserService;

pub struct UserController;

impl UserController {
    pub async fn get_users(req: Req) -> AppResult<Res> {
        let db = req.db();
        let users = UserService::fetch_users(&db).await?;
        Ok(Res::json(&users))
    }
}