use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use api::infra::endpoints::config_endpoints;
use api::configuracion::leer_configuracion;
use api::configuracion::Config;

use api::modules::proyectos::infra::endpoints::config_endpoints_proyectos;


// nombre api: api 
// paradigma: arquitectutura hexagonal
// .env: si, [puerto, host]
// archivo de endpoinits: api/src/infra/endpoints.rs fn = [config_endpoints]
// 

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Se le la configuración donde se obtienen las variables de entorno necesarias
    let configuracion: api::configuracion::Config = leer_configuracion();

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api/v1")
                .configure(config_endpoints)
                .configure(config_endpoints_proyectos)
        )
    })
    .bind((configuracion.host, configuracion.port))?
    .run()
    .await
}