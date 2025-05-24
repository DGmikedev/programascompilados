
use aes_gcm::{ Aes256Gcm, Nonce, Key };
use aes_gcm::aead::{Aead, KeyInit };  
use std::fs::File;
use std::io::Read;
use std::fs;

// AeadCore ,, OsRng ,, rand::RngCore;


/**
* Función que encripta archvios
* 
* input_path: ruta del archvio que se desea encriptar
* output_path: ruta donde se colocará el archivo encriptado
* key_bytes:  clave de seguridad
* iv: nonce(iv) clave 12 bytes
* 
* clave_de_seguridad: debe tener 32 bytes (256 bits). En apps profesionales, se obtiene de un KDF como PBKDF2 o derivado de una passphrase segura.
* Nonce(IV): vector aleatorio de 12 bytes. Debe ser único por clave para evitar vulnerabilidades.
* Se guarda junto con el archivo cifrado: el nonce no necesita ser secreto, pero sí único.
* Output: archivo con los primeros 12 bytes como nonce seguido del ciphertext.
*
 */
pub fn encriptador_de_archivos(input_path: &str, output_path: &str, key_bytes: &str, iv: &str)-> Result<(), Box<dyn std::error::Error>>
{

    // validamos que la clave sea de 32 bytes (AES-256)
    assert_eq!(key_bytes.len(), 32 );

    // leemos el archivo
    let mut plaintext = File::open(input_path).unwrap();

    let mut to_string = String::new();

    plaintext.read_to_string(&mut to_string).unwrap();

    // creamos un cifrador , con la clave
    let key = Key::<Aes256Gcm>::from_slice(key_bytes.as_bytes()); // key
    let cipher = Aes256Gcm::new(key);                             // cifrador 
    let nonce = Nonce::from_slice(iv.as_bytes());                 // noce o iv

    // ciframos el texto
    let ciphertext = cipher.encrypt(nonce, to_string.as_ref()).map_err(|e| format!("Posible error en iv ó en el archivo a encriptar {:?}", e))?;

    // guardamos el nonce (12 bytes) + cipher
    let mut encrypted_data = iv.as_bytes().to_vec();
    encrypted_data.extend(ciphertext);

    // escribimos el data encriptado en el path indicado
    let _ = fs::write(output_path, encrypted_data);

    Ok(())

}

/**
* Función que descencripta archvios
* 
* input_path: ruta del archvio encriptado
* output_path: ruta donde se colocará el archivo desencriptado
* key_bytes:  clave de seguridad
* iv: nonce(iv) clave 12 bytes
* 
* clave_de_seguridad: debe tener 32 bytes (256 bits). En apps profesionales, se obtiene de un KDF como PBKDF2 o derivado de una passphrase segura.
* Nonce(IV): vector aleatorio de 12 bytes. Debe ser único por clave para evitar vulnerabilidades.
* Se guarda junto con el archivo cifrado: el nonce no necesita ser secreto, pero sí único.
* Output: archivo con los primeros 12 bytes como nonce seguido del ciphertext.
*
 */
pub fn desencriptador_de_archivos(input_path: &str, output_path: &str, key_bytes: &str, iv: &str)->Result<(), Box<dyn std::error::Error>>
{
    // validamos que la clave sea de 32 bytes (AES-256)
    assert_eq!(key_bytes.len(), 32 );

    // leemos el archivo
    let encryptedtext = fs::read(input_path).map_err(|e| format!("Erroral leer el archivo encriptado {:?}", e))?;

    // comprobamos que no sea menor a 12, dado que al encriptar
    // se adjunta el IV en el archivo encriptado y si es menor a 12
    // significa que el IV no se podrá leér
    if encryptedtext.len() < 12 {
        return Err("Archivo con Nonce(IV) comprometido o incompleto".into());
    }

    // Extraer nonce(IV) y texto cifrado
    let (_nonce_bytes, ciphertext) = encryptedtext.split_at(12);

    let key = Key::<Aes256Gcm>::from_slice(key_bytes.as_bytes()); // key
    let cipher = Aes256Gcm::new(key);                             // cifrador 
    
    // noce o iv se adapta del string recibido
    let nonce = iv.as_bytes().into();                             

    // // deciframos 
    let textodesencriptado = cipher.decrypt(nonce, ciphertext.as_ref()).map_err(|e| format!("Posible error en iv ó el texto fue cifrado modificado {:?}", e))?;

    // escribimos el data desencriptada en el path indicado
    let _ = fs::write(output_path, textodesencriptado);

    Ok(())
}



fn main() {

    // 32 caracteres para aes-256
    let key = "*_(U}zJy[1lKxBBr-iGo`H>P<^y;5+fk"; // [0u8; 32]; // clave segura de 256 bits

    // 12 caracteres para iv
    let iv = "cFvEr4v8xzZd";

    encriptador_de_archivos("mensaje.txt", "mensaje.enc", &key, &iv).expect("Falló el cifrado");
    desencriptador_de_archivos("mensaje.enc", "mensaje_decrypted.txt", &key, &iv).expect("Falló el descifrado");
}




















// ENCRIPTACION SIMÉTRICA
// DES, 3DES, AES  AES es la que se recomendo para este proyecto
// para futuras en criptaciones TLS/SSL, HTTPS, PGP => RSA, ECC

/*
use aes_gcm::{ aead::{Aead, KeyInit, OsRng}, Aes256Gcm, Nonce};

use base64::{engine::general_purpose, Engine as _};

fn main(){

    // Este debe de ser Random
    let master = "12341234123412341234123412341234".as_bytes();
    let cipher = Aes256Gcm::new_from_slice(master).unwrap();

    let nonce_str_base64 = "fvXWsQhmTtDJsGFv";
    let nonce_str: Vec<u8> = general_purpose::STANDAR.decode(nonce_str_base64).unwrap();
    let nonce = Nonce::from_slice(&nonce_str_base64);

    // encrypted text from sender
    let ciphertext_base64 = "Zf5aB0bbVGGX3k9Yt6x+9daxCGZO0MmwYW8VUsOY4j3gNYXP47hvfGgd";
    let ciphertext = general_purpose::STANDARD.decode(ciphertext_base64).unwrap();

    // gets aead::Error here
    match cipher.decrypt(nonce, ciphertext.as_slice()) {
        Ok(decrypted) => {
            let result = String::from_utf8(decrypted).unwrap();
            println!("result: {}", result);
        }
        Err(err) => print!("{}", err), // <--- prints error: aead::Error
    };



} */


/*

use aes_gcm::{ aead::{Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm, Nonce, Key };


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path: String = String::from("./outenc/");
    let mut nombre: &str = "salida.enc";

    // Se genera la llave con valores random
    // un 
    let key = Aes256Gcm::generate_key(OsRng);  // KEY
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);  // IV

    let cipher = Aes256Gcm::new(&key);

    let ciphertext = cipher.encrypt(&nonce, b"Mensaje Sin Cifrar".as_ref()).map_err(|e| format!("Encrypt error: {:?}", e))?;
    
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).map_err(|e| format!("Decrypt error: {:?}", e))?;

   
    // println!("nonce: {:?}", nonce);

    match String::from_utf8(ciphertext.to_vec()) {
        Ok(string) => println!("{}", string),
        Err(error) => println!("Error: {:?}", error),
    }

    // println!("Key: {:?}", key);
    // println!("Mensaje cifrado: {:?}", ciphertext);
    // println!("Mensaje decifrado: {:?}", plaintext);

    // assert_eq!(&plaintext, b"Mensaje Sin Cifrar");

    Ok(())
}
*/