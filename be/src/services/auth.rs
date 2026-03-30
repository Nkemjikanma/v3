use crate::common::{
    utils::{PasswordUtils, JWT},
    valid_string_entry::ValidStringEntry,
};
use crate::errors::AppError;
use crate::types::{app::AppState, auth::LoginForm};
use secrecy::ExposeSecret;
pub struct AuthService;

impl AuthService {
    #[tracing::instrument(name = "signing in", skip(login_body, app_state))]
    pub async fn login(login_body: LoginForm, app_state: &AppState) -> Result<String, AppError> {
        let LoginForm { username, password } = login_body;

        let validated_username =
            ValidStringEntry::parse(username).map_err(|e| AppError::ValidationError(e))?;
        let validated_password = ValidStringEntry::parse(password)
            .map_err(|e| AppError::ValidationError(e))?
            .as_ref()
            .to_string();

        if validated_username.as_ref().to_lowercase().to_string()
            != app_state
                .app_config
                .application
                .admin_username
                .expose_secret()
                .to_lowercase()
                .to_string()
        {
            tracing::warn!("Invalid login attempt");

            return Err(AppError::InvalidUserCredentials);
        }

        if !PasswordUtils::verify_password(
            &validated_password,
            app_state
                .app_config
                .application
                .admin_password_hash
                .expose_secret(),
        ) {
            tracing::warn!("Invalid login attempt");

            return Err(AppError::InvalidUserCredentials);
        }

        tracing::info!("Login successful");

        // generate JWT
        let token = JWT::generate_token(
            &validated_username.as_ref(),
            &app_state.app_config.application.jwt_secret.expose_secret(),
        )
        .map_err(|_| {
            tracing::error!("Error generating jwt");

            AppError::JWTCreationFailed
        })?;

        tracing::info!("JWT successfully created for");

        // Ok(user::LoginResponse {
        //     token,
        //     username: request.username,
        //     user_id: user.id,
        // })

        Ok(token)
    }
}
