use crate::controllers;
use actix_web::web;
pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(controllers::landing::index)
        .service(controllers::landing::about)
        .service(controllers::tarif::index)
        .service(controllers::jukir::index)
        .service(controllers::parkir::index);
}
