use crate::common::api_response::{APIResponse, AppResponse};
use crate::services::auth::AuthService;
use crate::types::auth::LoginForm;
use actix_web::web;
pub async fn login(login_body: web::Json<LoginForm>) -> AppResponse<String> {
    let token = AuthService::login(&login_body.into_inner()).await?;

    Ok(APIResponse::success(token.to_string()))
}
