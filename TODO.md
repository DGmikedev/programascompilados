# APIS EN RUST Y ACTIX-WEB

```rust
use actix_files::NamedFile;
use actix_web::{ 
   cookie::time::Duration, dev::ServiceRequest, get, http::header::{self, ContentDisposition}, post, web::{self, Json}, App, Error, HttpRequest, HttpResponse, HttpServer, Responder
};


use actix_web::error;

use actix_multipart::{form::MultipartFormConfig, Multipart};                                  // para subir un archivo
use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};  // para subir más de un archivo

use std::path::PathBuf;
use futures_util::{future::ok, TryStreamExt};
use std::io::Write;
use serde::{Deserialize, Serialize};

use actix_web_httpauth::{                     // para autenticación básica
    extractors::{
        basic::{self, BasicAuth},
        AuthenticationError
    },
    headers::www_authenticate
};
use actix_web_httpauth::middleware::HttpAuthentication;


use actix_web_httpauth::extractors::bearer::BearerAuth;  // token refrehs  ///


// para jwt
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation
};

use chrono::{Utc, Duration as duration_chrono};




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
///  
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

//////////////////////////////////////////////////////////////////////////////

// COMPROBACION PARA SUBIR UN SOLO ARCHIVO  /////////////////////////////////////////////////////

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

//////////////////////////////////////////////////////////////////////////////////////////

// COMPROBACION PARA SUBIT MULTIPLES ARCHIVOS  //////////////////////////////////////////
//  IMPORTANTE 50 MB para todo el PAYLOAD  !!!!!!!!!!!!  ///////////////////////////////

#[derive(Debug, MultipartForm)]
pub struct FormularioArchivos{
    pub nombre: Text<String>,
    pub apellido: Text<String>,
    pub archivo: TempFile,
    pub anexos: Vec<TempFile>
}

#[post("/subirarchivos")]
async fn formulario_multipart(MultipartForm(form): MultipartForm<FormularioArchivos>) -> Result<HttpResponse, Error>{
    println!("Nombre: {}", form.nombre.as_str());
    println!("Apellido: {}", form.apellido.as_str());
    let file_name = form.archivo.file_name.unwrap();
    let file_path = format!("./archivos/{file_name}");
    form.archivo.file.persist(file_path).unwrap();
    Ok(HttpResponse::Ok().body("Formulario procesado correctamente"))
}

///////////////////////////////////////////////////////////////////////////////////////////////

// PRUEBA PARA COMPROBAR EL DESPACHADO DE ARCHIVOS ESTATICOS  /////////////////////////////////

#[get("/estaticos/{nombre_archivo}")]
async fn archivo_estaticio(req: HttpRequest)->Result<NamedFile, Error>{

    let pbuf: PathBuf = req.match_info().query("nombre_archivo").parse().unwrap();

    let mut ruta = pbuf.into_os_string().into_string().unwrap();

    println!("Buscando archivo en: {}", ruta.clone());

    ruta = format!("./estaticos/{ruta}");

    let archivo_disponible: NamedFile = NamedFile::open(&ruta)?;

    // Ok(archivo_disponible.use_last_modified(true)) // solo para mostrar/disponer en el navegador

    // para que el archivo del cual se quiere disponer se descargue desde el navegador
    Ok( archivo_disponible
        .set_content_disposition( ContentDisposition::attachment( ruta.clone() ) )
        .use_last_modified(true) 
    )


}


// COMPROBACION DE AUTENTICACIÓN BÁSICA ///////////////////////////////////////////////////////

// 1.-  SE IMPORTAN LOS PAQUETES actix-web actix-web-httpauth
// 2.-  Se crean los scopes para aglutinar los recursos entre rutas
//      si no se hiciera así, el middleware revisaría todas las rutas y no dejaria pasar las 
//      peticiones en rutas publicas

// Extractor reciven las peticiones http y extraen la información


// middleware que valida las credenciales

async fn validador(
    req: ServiceRequest, 
    credenciales: BasicAuth)
    ->Result<ServiceRequest, (Error, ServiceRequest)>{

    if credenciales.user_id() == "admin" && credenciales.password().unwrap() == "admin" {
        Ok(req)
    }else{
        Err(
            ( AuthenticationError::new(www_authenticate::basic::Basic::default()).into(),
            req
        ))
    }

}


#[get("/publico")]
async fn publico()->HttpResponse{
    HttpResponse::Ok().body("Acceso a publico")
}


#[get("/privado")]
async fn privado(auth: BasicAuth)->Result<HttpResponse, Error>{
    if auth.user_id() == "user" && auth.password().unwrap() == "123" {
         Ok(HttpResponse::Ok().body("Acceso a privado"))
    }else{
        Err(AuthenticationError::new(www_authenticate::basic::Basic::default()).into())
    }
   
}



#[get("/confidencial")]
async fn confidencial()->HttpResponse{
    HttpResponse::Ok().body("Acceso a información confidencial")
}


#[get("/super_confidencial")]
async fn super_confidencial()->HttpResponse{
    HttpResponse::Ok().body("Acceso a información super_confidencial")
}

///////////////////////////////////////////////////////////////////////////////////////////////

// COMPROBACION DEL FUNCIONAMIENTO DE JWT (JSON WEB TOKEN) ///////////////////////////////////

// 1.-  EL TOKEN ESTA FORMADO POR TRES PARTES  ENCABEZADO, PAYLOAD Y FIRMA
// ejemplo de jwt: 
// ENCABEZADO                           PAYLOAD                                                                                     FIRMA
// eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.KMUFsIDTnFmyG3nMiGM6H9FNFUROf3wh7SmqJp-QV30
// INFO DESCRIPCIÓN DE JWT :: https://datatracker.ietf.org/doc/html/rfc7519
// 2.-  SE IMPORTA EL CRATE   cargo add jsonwebtoken, cargo add chrono


// esta constante va en una variable de entorno y en un .env
const LLAVE: &[u8] = b"12345";

// estructura que conformará el json web token
#[derive(Deserialize, Serialize, Debug)]
struct Claims{
    iss: String,
    sub: String,
    exp: usize,
    iat: usize,
    tipo: String,  /// refresh token
    user_id: usize,
}


/// COPROBACIÓN DEL REFRESCO DEL TOKEN ////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize)]
struct LoginForm{
    usuario: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Loginresult{
    token: String,
    refresh: String,
}

#[derive(Serialize, Deserialize)]
struct refreshResult{
    token:String
}
///////////////////////////////////////////////////////////////////////////////////////////////



fn generar_token(iss: String, sub:String, minutos: i64, user_id: usize, tipo: String)-> String{
    let header =  Header::new(Algorithm::HS512);
    let encoding_key = EncodingKey::from_secret(LLAVE);

    let exp: usize = (Utc::now() + duration_chrono::minutes(minutos)).timestamp() as usize;
    // let exp: usize = 0usize; // SE COMPRUEBA QUE LE TIMEPO DE EXPIRACIÓN SE GENERA SI SE RETARDAN LAS RESPUESTAS [Error(ExpiredSignature)]
    let iat: usize = Utc::now().timestamp() as usize; 

    let my_claims:Claims = Claims {
        iss,
        sub,
        exp,
        iat,
        tipo, /// refresh token
        user_id
    };

    encode(&header, &my_claims, &encoding_key).unwrap()
}

fn validar_token(token: String)-> Result<Claims, jsonwebtoken::errors::Error>{
    let validacion = Validation::new(Algorithm::HS512);
    let decoding_key = DecodingKey::from_secret(LLAVE.as_ref());
    // let decoding_key = DecodingKey::from_secret(b"dfdf".as_ref());    // SE COMPRUEBA QUE SE DEBE DE TENER LA MISMA LLAVE, MUESTRA:  [Error(InvalidSignature)]

    let resultado = decode::<Claims>(&token, &decoding_key, &validacion);

    match resultado {
        Ok(c) => {
            println!("Token Valido");
            Ok(c.claims)
        },
        Err(e) => {
            println!("Token invalido");
            Err(e)
        }
    }
}

// función para validar token refresh ///////////////////////////////////////////////////////


async fn validador_token_refresh(
    req: ServiceRequest, 
    credenciales: Option<BearerAuth>)
    ->Result<ServiceRequest, (Error, ServiceRequest)>{

    let Some(credenciales) = credenciales else {
        return Err((error::ErrorBadRequest("No se especificó el token"), req));
    };

    let token = credenciales.token();

    let resultado = validar_token(token.to_owned());

    match resultado {
        Ok(claims) =>{ 
            println!("Los claims son: {:?}", claims);
            if claims.tipo != "refresh" {
              return ok(req);
            }else{
                return Err((error::ErrorForbiden("No tiene acceso"), req));
            }
        
    }
        Err(e) => println!("Error el token no es valido: {:?}", e)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////



// funcion main
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //  COMPROBACIÓN DE LA GENERACIÓN DE JWT //////////////////////////////////////////////////

    let iss: String = "rusty mike".to_string();
    let sub: String = "test_jwt" .to_string();
    let minutos: i64 = 5 as i64;
    let user_id: usize = 1; 
    let tipo:String = "".to_string();

    let token:String = generar_token(iss, sub, minutos, user_id, tipo);

    println!("Token Generado: {}", &token);

    match validar_token(token){
        Ok(c) => {
            println!("Token: {:?}", c);
        },
        Err(e)=>{
            println!("Error [{:?}]", e);
        }
    }

    
    /* (&str, u16)
       (IpAddr, u16)
       (Ipv4Addr, u16)
       (Ipv6Addr, u16)
       (String, u16) */
    
    let ip: &str = "127.0.0.1";  //   (&str, u16)
    let puerto: u16 = 8080;      //    ip , port

    HttpServer::new(|| {
        // COMPROBACIÓN DE PARA AUMENTAR EL TAMAÑO DE LOS PAYLOADS EN LAS SUBIDAS DE ARCHIVOS  ///////
        
        // SE ESTABLECE EL LIMTE DE MEMORIA TOTAL Y DE PAYLOAD IGUAL
        // 1024 = 1 Kilobyte
        // 1024 * 1024 = 1 Mega Byte
        // 1024 * 1024 * 100 = 100 Mega bytes
        let memoria_para_payload: usize = 1024 * 1024 * 100;
        let multipart_form_config: MultipartFormConfig = MultipartFormConfig::default()
            .total_limit(memoria_para_payload)
            .memory_limit(memoria_para_payload);
        //////////////////////////////////////////////////////////////////////////////////////////////
        
        /// CREACIÓN DE LA VARIABLE QUE DA DE ALTA EL MIDDLEWARE   ///////////////////////////////////

        let auth_in  =  HttpAuthentication::basic(validador);
        ////////////////////////////////////////////////////////////////////////////////////////////////
        
        let auth_basic = basic::Config::default().realm("privado");
        
        App::new()
        // .wrap(auth_in)                       // se da de alta el validador de credenciales prueba que se puede aplicar autorización a todas las rutas
        .app_data(multipart_form_config)        // configuración del manejador de archivos multiparte
        .app_data(auth_basic)                   // configuración de la autenticación
        .route("/", web::get().to(estatus))     // alternativa a ruta
            .service(usuario)
            .service(formulario)
            .service(usuariof)
            .service(qp_usuario)
            .service(suma)
            .service(person)
            .service(responsejson)
            .service(subirarchivo)
            .service(formulario_multipart)
            .service(archivo_estaticio)
            // bloque que comprueba las credenciales ///////////////////////
            .service(publico)                        //  esta ruta queda sin autorización 
            .service(                               
                web::scope("/admin")                 // este espacio o ambito (scope) tiene dos rutas  
                .wrap(auth_in)                       // se incluye el middleware creado anteriormente y engloba las rutas que contiene ele servicio 
                .service(super_confidencial)         // esta ruta esta revisada de autorización con admin admin 
                .service(confidencial)               // esta ruta esta revisada de autorización con admin admin  
            )
            .service(privado)                         // esta ruta esta revisad ade autorización user - 123 por que esta fuera del ambito (scope) /admin
            //////////////////////////////////////////////////////////////////
    })
    .bind((ip, puerto))?
    .run()
    .await
}


```
