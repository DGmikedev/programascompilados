use rand::{distributions::Alphanumeric, Rng};
use std::fs;

/**
* Función que genera claves alfanúmericos
*
* usize cant: numero de caracteres deseado en clave
* 
* return (bool, String)
*/
pub fn genstring(cant: usize)->(bool,String){
    let cadena: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(cant)
        .map(char::from)
        .collect();
    (true, cadena)
}

/**
* Función que genera un documento con las claves visibles y compresibles
* en formato XML o TXT
*
* String path: String = path donde se guardará el archivo creado
* String key: dato que se registra entre las etiquetas <key> 
* String iv:  dato que se registra entre las etiquetas <iv> 
* 
* return (bool, String)
*/
pub fn gendocs(path: String, key: String, iv: String)->(bool, String){
    
    let stringout = format!("<key>{}</key>\n<iv>{}</iv>", key, iv);

    let out = fs::write(&path, stringout);

    let outd:(bool, String) = match out{
        Ok(_salida) => (true, "Creacion de docuemnto exitosa en el path: [".to_string() + &path + "]"),
        Err(_e) => (false, _e.to_string()),
    };
    outd
}