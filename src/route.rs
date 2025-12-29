use crate::controllers;
use actix_web::web;
pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(controllers::landing::index)
        .service(controllers::landing::about)
        .service(controllers::tarif::index)
        .service(controllers::jukir::index)
        .service(controllers::jukir::create)
        .service(controllers::jukir::change)
        .service(controllers::jukir::remove)
        .service(controllers::parkir::index)
        .service(controllers::parkir::create)
        .service(controllers::parkir::change)
        .service(controllers::parkir::remove)
        .service(controllers::transaksi::index)
        .service(controllers::transaksi::create)
        .service(controllers::transaksi::change)
        .service(controllers::transaksi::remove);
}
