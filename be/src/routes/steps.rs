use crate::handlers::steps;
use crate::middleware::validator;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn configure_steps(cfg: &mut web::ServiceConfig) {
    let auth_middleware = HttpAuthentication::bearer(validator);
    cfg.service(
        web::scope("/steps")
            .route("", web::get().to(steps::get_steps))
            .service(
                web::scope("/steps")
                    .wrap(auth_middleware)
                    .route("", web::post().to(steps::set_steps)),
            ),
    );
}
