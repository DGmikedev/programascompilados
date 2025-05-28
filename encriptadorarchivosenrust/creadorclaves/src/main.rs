mod encriptador;
mod generadordocumento;
mod generadorllaves;

fn main() {
    

    // crear documento con las llaves  /////////////////////////////
    // las llaves solo pertenece al dueño del servidor
    ////////////////////////////////////////////////////////////////

    // En servidor donde vive el aplicativo
    let namedoc = "serv85-95";                                  // nombre descriptivo de las llaves 
    let pathout = "keys/";                                      // path con permisos de lectura controlada
    let pathoutenc = "encrptkeys/";                             // path con permisos semicontrolados solo devs en desarrollando con permisos y/producción
    let path = format!("{}{}.xml",pathout,namedoc);             // creación de path xml con llaves visibles
    let pathenc = format!("{}enc_{}.enc", pathoutenc,namedoc ); // creación de path con archivo encriptado
    let pathver = format!("{}{}.txt", pathout, namedoc);        // path de creación del docuemnto para comprobar la correcta creación del docuemnto xml
                                                                // no olvidar borrar este docuemnto de prueba 

    // se genera la clave longitud 32 caracteres por seguridad
    let key = generadorllaves::genstring(32);
    
    // se genera iv longitud 12 caracteres por seguridad
    let iv_nonce = generadorllaves::genstring(12);

    // se crea el documento xml donde se muestra la llave
    // solo tienen acceso el dueño del sercidor
    let tuplakeys:(bool, String) = generadorllaves::gendocs(path.to_string() , key.clone(), iv_nonce.clone());


    // encriptar el documento con las llaves  ///////////////////////
    // el archivo encriptado es el que es visible al equipo 
    // de desarrollo
    /////////////////////////////////////////////////////////////////

    // si se genera correctamente la llave se encripta
    if tuplakeys.0 {
        // println!("Llaves generadas correctamente: {}", path.clone());
        // println!("key: {}\niv: {}\npathxml: {}\npathenc: {}", key, iv_nonce, path.clone(), pathenc.clone());
        encriptador::encriptador_de_archivos(&path.clone(), &pathenc.clone(), &key, &iv_nonce)
        .expect("Fallo el encriptador");

        // test de verificación de archivo 
        // Si el encriptado con las credenciales es fucnional
        // se creará un documento igual al de las llaves
        // pero con extención .txt
        // NOTA: NO OLVIDAR BORRAR ESTE DOCUMENTO
        encriptador::desencriptador_de_archivos(&pathenc.clone(), &pathver, &key, &iv_nonce )
        .expect("Falló el desencriptador");
        
    }

    // creador de documento log ////////////////////////////////////////
    // se crea el log para el guardado de datos en historial de repaldo
    // la data del documento se ajusta en cada aplictivo
    /////////////////////////////////////////////////////////////////////
    
    generadordocumento::documento();

}
