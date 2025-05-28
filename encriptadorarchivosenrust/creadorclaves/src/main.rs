mod encriptador;
mod generadordocumento;
mod generadorllaves;

fn main() {
    
    encriptador::test_escriptar();
    
    let key = "*_(U}zJy[1lKxBBr-iGo`H>P<^y;5+fk";
    let iv = "cFvEr4v8xzZd";
    encriptador::encriptador_de_archivos("mensaje.txt", "mensaje.enc", &key, &iv).expect("Falló el cifrado");
    encriptador::desencriptador_de_archivos("mensaje.enc", "mensaje_decrypted.txt", &key, &iv).expect("Falló el descifrado");
    
    generadordocumento::documento();


    let key = generadorllaves::genstring(32);
    let iv_nonce = generadorllaves::genstring(12);
    let namedoc = "serv_name_85-95.xml";
    let pathout = "keys/";
    let path = format!("{}{}",pathout,namedoc);

    let tupla:(bool, String) = generadorllaves::gendocs(path.to_string() ,key, iv_nonce);

    println!("{:?}", tupla);

    println!("Hello, world!");
}
