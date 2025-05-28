mod encriptador;
mod generadordocumento;
mod generadorllaves;

fn main(){

    // se inicia string con iformación a mostrar   
    let mut outext: String = String::new();
    outext.push_str("\n------------------------------------------------------\n");

    ////////////////////////////////////////////////////////////////
    //      crear documento con las llaves                        //
    //      las llaves solo pertenece al dueño del servidor       //
    ////////////////////////////////////////////////////////////////
    
    // se genera la clave longitud 32 caracteres por seguridad
    let key = generadorllaves::genstring(32);
        if key.0 {
            outext.push_str("- Creación exitosa de llave con 32 caracteres\n");
        }else{
            outext.push_str("- Fallo la creación de llave con 32 caracteres\n");
        }

    // se genera iv longitud 12 caracteres por seguridad
    let iv_nonce = generadorllaves::genstring(12);
        if iv_nonce.0 {
            outext.push_str("- Creación exitosa de iv con 12 caracteres\n");
        }else{
            outext.push_str("- Fallo la creación de iv con 12 caracteres\n");
        }
    
    let namedoc = "serv85-95";  // nombre descriptivo de las llaves 
    let pathout = "keys/";      // path con permisos de lectura controlada
    let pathKeys = format!("{}{}.xml",&pathout,namedoc); // creación de path xml con llaves visibles

    // se crea el documento xml donde se muestra la llave
    // solo tienen acceso el dueño del servidor
    let tuplakeys:(bool, String) = generadorllaves::gendocs(pathKeys.to_string() , key.1.clone(), iv_nonce.1.clone());


    /////////////////////////////////////////////////////////////////
    //      encriptar el documento de las claves de                // 
    //      las bases de datos con las llaves generadas            //
    //      el archivo encriptado es el que es visible al equipo   //
    //      de desarrollo                                          //
    /////////////////////////////////////////////////////////////////

    let nomclaves = "clavesBDts";      // nombre del archivo xml con las claves de las bases de datos
    let pathoutenc = "encrptkeys/";    // path con permisos semicontrolados solo devs en desarrollando
    let pathclaves = format!("{}{}.xml",&pathout, nomclaves);      // se crea path con las claves xml de las bases de datos
    let pathenc = format!("{}enc_{}.enc", pathoutenc,&nomclaves ); // creación de path con archivo encriptado
    let pathver = format!("{}{}.txt", pathout, &nomclaves);        // path de creación del docuemnto para comprobar la correcta creación del docuemnto xml

    // si se genera correctamente la llave se encripta
    if  tuplakeys.0 {

        // se encripta el archivo de las claves de bases de datos 
        // con las llaves generadas anteriormente
        encriptador::encriptador_de_archivos(&pathclaves.clone(), &pathenc.clone(), &key.1.clone(), &iv_nonce.1.clone())
        .expect("Fallo el encriptador");

        // test de verificación de archivo 
        // Si el encriptado con las credenciales es fucnional
        // se creará un documento igual al de las llaves
        // pero con extención .txt
        // NOTA: NO OLVIDAR BORRAR ESTE DOCUMENTO
        encriptador::desencriptador_de_archivos(&pathenc.clone(), &pathver, &key.1.clone(), &iv_nonce.1.clone() )
        .expect("Falló el desencriptador o creador de archivo TXT de comprobación");

        let msgenc = format!("- Instalación y encriptado de archivo de llaves correcto [{}]\n- Instalación y creación de archivo txt de comparación correcto [{}]\n", &pathenc.clone(), &pathver);

        outext.push_str(&msgenc);
    }

    ////////////////////////////////////////////////////////////////////
    //     creador de documento log                                   //
    //     se crea el log para el guardado de datos en                //
    //     historial de repaldo la data del documento se ajusta       //
    //     en cada aplictivo                                          //
    ////////////////////////////////////////////////////////////////////
    
    let docuemntogenerado = generadordocumento::documento();
    if docuemntogenerado.0 {
        let so = format!("- {}\n",&docuemntogenerado.1);
        outext.push_str(&so);
    }else{
        let so = format!("- {}\n",&docuemntogenerado.1);
        outext.push_str(&so);
    }

    outext.push_str("------------------------------------------------------\n");

    println!("{}", outext);


}
