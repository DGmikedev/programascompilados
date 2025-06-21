# APIS EN RUST Y ACTIX-WEB

```rust

use actix_web::{ 
    get, http::header::{self, ContentDisposition}, post, web::{self, Json}, App, Error, HttpResponse, HttpServer, Responder
};

use actix_multipart::Multipart;

use futures_util::TryStreamExt;
use std::io::Write;

use serde::{Deserialize, Serialize};


// COMPROBACIÓN DE PETICIONES  /////////////////////////////////////////////
async fn estatus()->HttpResponse {
    HttpResponse::Ok()
        .content_type(header::ContentType::html())
        .body("Servidor 127.0.0.1:8080 online....")
}

#[get("/usuario")]
async fn usuario()->HttpResponse{
    HttpResponse::Ok()
        .content_type(header::ContentType::json())
        .body(r#"
        {
            "id"     : 1,
            "nombre" : "Rusty",
            "appPat" : "Stack"
        }
        "#)
}

#[post("/formulario")]
async fn formulario()-> HttpResponse{
    HttpResponse::BadRequest()
        .content_type(header::ContentType::json())
        .body(r#"
        {
            "codigo" : "400",
            "mensaje" : "error",
            "campos" : [
                "campo" : "nombre",
                "descripcion" : "capo nombre no puede ser nulo"
            ]
        }
        "#)
}

///////////////////////////////////////////////////////////////////////////

// COMPROBACIÓN DE PETICIONES CON PARAMETROS //////////////////////////////

// .- Por medio de pathparams  /usuariof/{id}/{activo}
// .- con serde se deserializa los para,etros que se le pasan   use serde::Deserialize


// .- Esta estruct representa todos los parametros 
//   que usrá el path param y los guardará en la viable args
#[derive(Deserialize)]
struct Userparams{
      id : u8,
      activo: String  
}

#[get("/usuariof/{id}/{activo}")]
async fn usuariof(args: web::Path<Userparams>)->HttpResponse{
    HttpResponse::Ok().body(format!("Usuario id = {} , Estado de usuario = {}", args.id, args.activo)) // <-- aquí args

    // no recomendada 
    // async fn usuariof(args: web::Path<(u8, String)>)->HttpResponse{   <-- aquí la forma nativa sin serilize pero dificíl de mantener en el tiempo
    // let (id, estado) = args.into_inner();
    // HttpResponse::Ok().body(format!("Usuario id = {} , Estado de usuario = {}", id, activo)) // <-- aquí sin deserialize
}

// .-con query params  /usuariofQp?id=1&activo=true
#[get("/qp_usuario")]
async fn qp_usuario(args: web::Query<Userparams>)->HttpResponse{
    HttpResponse::Ok().body(format!("Usuario id = {} , Usuario activo = {}", args.id, args.activo)) // <-- aquí args

    // no recomendada 
    // async fn usuariof(args: web::Path<(u8, String)>)->HttpResponse{   <-- aquí la forma nativa sin serilize pero dificíl de mantener en el tiempo
    // let (id, estado) = args.into_inner();
    // HttpResponse::Ok().body(format!("Usuario id = {} , Estado de usuario = {}", id, activo)) // <-- aquí sin deserialize
}


// SERDE:  cargo add serde --features=derive adjuntar serde para serializar

///////////////////////////////////////////////////////////////////////////
 
// COMPROBACIÓN DEL USO DE JSON Y URL-ENCODED FORMS  //////////////////////

// .- Se crea la estructura de los datos para los parametros
// .- para etsa ruta se deserealiza un Json
//  que tiene esta forma | a esta ruta: "/suma", por el método POST 
// {                     v 
//     "sumando1" : 1,
//     "sumando2" : 10,
//     "sumando3" : "s"
// }

#[derive(Deserialize)]
struct Sumaparametros{
    sumando1 : u8,
    sumando2 : u8,
    sumando3 : u8
}

#[post("/suma")]
async fn suma(args: web::Json<Sumaparametros>)->HttpResponse{
    let res = args.sumando1 + args.sumando2 + args.sumando3;
    HttpResponse::Ok().body(format!("El resultado es: {}", res))
}


// URL-Encoded Forms

#[derive(Deserialize)]
struct Person{
    nombre: String,
    apellido: String,
    edad: u8
}

#[post("/person")]
async fn person(args: web::Form<Person>)->HttpResponse{
    HttpResponse::Ok().body(
        format!("Tu nombre es {} {} y tu edad es de {} anios", 
            args.nombre, args.apellido, args.edad) )
} 

//////////////////////////////////////////////////////////

// COMPROBACION DE RESPUESTA EN FORMATO JSON ////////////

#[derive(Serialize)]
struct JsonResponse{
    id: u8,
    nickname: String,
    departament: String,
    supervisa: Vec<AdicionARespuestaConJson>
}

#[derive(Serialize)]
struct AdicionARespuestaConJson{
    oficina: String,
    cubiculo: u8,
    computadoraId: u8
}

#[get("/rsponsejson")]
async fn responsejson()->HttpResponse{

    let facturacion:AdicionARespuestaConJson =  AdicionARespuestaConJson{
        oficina: "facturacion".to_string(),
        cubiculo: 15,
        computadoraId: 125
    };
    let exterior:AdicionARespuestaConJson =  AdicionARespuestaConJson{
        oficina: "ventas foraneas".to_string(),
        cubiculo: 5,
        computadoraId: 20
    };
    let jsn: JsonResponse = JsonResponse{
        id:1,
        nickname: "Rusty".to_string(),
        departament: "sells".to_string(),
        supervisa: vec![
            AdicionARespuestaConJson{oficina:"gerencia".to_string(), cubiculo:8, computadoraId:200 },
            facturacion,
            exterior]  
    };

    HttpResponse::Created().json(jsn)  // created es un estatus code 201
  
}  
/*  FORMA PARA ENVIAR SOLO EL JSON SINESTATUS
    PARA ASIGNAR UN ESTATUS SE DEBE USAR HttpResponse 
async fn responsejson()->impl Responder{

    let jsn = JsonResponse{
        id:1,
        nickname: "Rusty".to_string(),
        departament: "sells".to_string()
    };

    web::Json(jsn)

    // HttpResponse::Ok().body()
}  */

/////////////////////////////////////////////////////////////////////////////////////////

// COMPROBACION PARA SUBIR ARCHIVOS  /////////////////////////////////////////////////////

// .- CREATE PARA MULTIPART EN ACTIX
// .- cargo add actix-multipart

// .- cargo add futures-util      <- para hacer operaciones asincronas

// el archivo se suben en el path:  apis\rustactix\apirust\archivos


#[post("/subir")]
async fn subirarchivo(mut payload: Multipart)-> Result<HttpResponse, Error>{
    
    
    while let Some(mut field) = payload.try_next().await? {   // solo con future-utils

        let content_disposition = field.content_disposition();   // 

        let file_name = content_disposition.expect("NO SE PUDO ACCEDAR AL NOMBRE DEL ARCHIVO") // obtener el nombre del archivo
            .get_filename()
            .unwrap();  

        let file_path = format!("./archivos/{file_name}");     // ruta a la carpeta donde se guardarán los archivos a subir
        let mut archivo = web::block( || std::fs::File::create(file_path))// web::block bloquea el sistema mientras File crea el archivo
            .await??;    

        while let Some(chunk) = field.try_next().await?{  // parte los chunks (cachos de archivo)
            archivo = web::block(move || archivo.write_all(&chunk) // graba cada chunk en el archivo
            .map(|_|archivo))
            .await??; 
        }
    } 

    Ok(HttpResponse::Ok().body("Archivo subido correctamente"))
    
}





/////////////////////////////////////////////////////////////////////////////////////////

// funcion main
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    
    /* (&str, u16)
       (IpAddr, u16)
       (Ipv4Addr, u16)
       (Ipv6Addr, u16)
       (String, u16) */
    
    let ip: &str = "127.0.0.1";  //   (&str, u16)
    let puerto: u16 = 8080;      //    ip , port



    HttpServer::new(|| {
        App::new()
            // .service(status)
            // .service(echo)
            .route("/", web::get().to(estatus)) // alternativa a ruta
            .service(usuario)
            .service(formulario)
            .service(usuariof)
            .service(qp_usuario)
            .service(suma)
            .service(person)
            .service(responsejson)
            .service(subirarchivo)
    })
    .bind((ip, puerto))?
    .run()
    .await
}
```