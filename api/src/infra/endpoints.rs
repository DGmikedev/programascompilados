use actix_web::{get,  web,  Responder, HttpResponse};

#[get("/version")]
async fn version()->impl Responder{

    let 

    HttpResponse::Ok().body("1.0.0")
}


pub fn config_endpoints(cfg: &mut web::ServiceConfig){
     cfg.service(version);
}