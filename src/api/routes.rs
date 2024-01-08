use crate::api::task;
use actix_web::web::ServiceConfig;

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(task::find_all);
    cfg.service(task::find);
    cfg.service(task::create);
    cfg.service(task::update);
    cfg.service(task::delete);
}
