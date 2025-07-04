use actix_web::{get,  web,  Responder, HttpResponse};
use serde;
use serde_json;


#[get("/proyectos")]
async fn proyectos()->impl Responder{

    //  

    /*
    #[derive(Debug)]
pub struct Proyecto{
    pub id: u16,
    pub nombre: String,
    pub departamento: String
}

#[derive(Debug)]
pub struct Usuario{
    pub id: u16,
    pub nombre_us: String,
    pub departamento_us: Vec<u16>
}
     */


    HttpResponse::Ok().body("1.0.0")
}


pub fn config_endpoints_proyectos(cfg: &mut web::ServiceConfig){
     cfg.service(proyectos);
}