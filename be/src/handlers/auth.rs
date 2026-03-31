use crate::common::api_response::{APIResponse, AppResponse};
use crate::services::auth::AuthService;
use crate::types::app::AppState;
use crate::types::auth::LoginForm;
use actix_web::web;
use std::sync::Arc;

pub async fn login(
    login_body: web::Json<LoginForm>,
    app_state: web::Data<Arc<AppState>>,
) -> AppResponse<String> {
    let token =
        AuthService::login(login_body.into_inner(), &app_state.app_config.application).await?;

    Ok(APIResponse::success(token.to_string()))
}
