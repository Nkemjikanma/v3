use crate::common::utils::JWT;
use crate::errors::AppError;
use crate::types::app::AppState;
use actix_web::{dev::ServiceRequest, web, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use secrecy::ExposeSecret;
use std::sync::Arc;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();

    // Get jwt_secret from app state
    let app_state = req
        .app_data::<web::Data<Arc<AppState>>>()
        .expect("AppState not found");

    let secret = app_state.app_config.application.jwt_secret.expose_secret();

    match JWT::verify_token(token, secret) {
        Ok(_claims) => Ok(req),
        Err(_) => Err((AppError::InvalidToken.into(), req)),
    }
}
