use actix_web::web;

use crate::handlers::{healthz::{liveness_handler, readiness_handler}, ziwei};

pub fn health_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(readiness_handler).service(liveness_handler);
}

pub fn ziwei_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(ziwei);
}