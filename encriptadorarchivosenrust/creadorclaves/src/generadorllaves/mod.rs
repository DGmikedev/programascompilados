use rand::{distributions::Alphanumeric, Rng};
use std::fs;


pub fn genstring(cant: usize)->String{
    let cadena: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(cant)
        .map(char::from)
        .collect();
    cadena
}

pub fn gendocs(path: String, key: String, iv: String)->(bool, String){
    
    let stringout = format!("<key>{}</key>\n<iv>{}</iv>", key, iv);

    let out = fs::write(&path, stringout);

    let out:(bool, String) = match out{
        Ok(_salida) => (true, "Creacion exitosa:".to_string() + &path),
        Err(_e) => (false, _e.to_string()),
    };
    out
}