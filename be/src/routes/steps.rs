use crate::handlers::steps;
use actix_web::web;

pub fn configure_steps(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/steps")
            .route("", web::get().to(steps::get_steps))
            .route("", web::post().to(steps::set_steps)),
    );
}
