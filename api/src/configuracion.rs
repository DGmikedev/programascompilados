use dotenv::dotenv;
use std::env;

pub struct Config{
    pub port: u16,
    pub host: String
}

pub fn leer_configuracion()-> Config{
    let resp = dotenv().ok();
    
    if resp.is_some() {
        println!("[ARCHVIO DE CONFIGURACIÓN .env CARGADO: [{:?}] ]", resp);
    }else{
        panic!("[ ERROR AL LEER EL ARCHIVO DE CONFIGURACÓN .env ]");
    }
    
    Config{
        port: env::var("PORT").expect("[NO HAY PUERTO DEFINIDO PARA: api/v1]").parse().expect("[MALA DEFINICIÓN DE PUERTO api: api/v1]"),
        host: env::var("HOST").expect("[NO HAY HOST DEFINIDO PARA: api/v1]"),
    }
}