// ENCRIPTACION SIMÉTRICA
// DES, 3DES, AES  AES es la que se recomendo para este proyecto
// para futuras en criptaciones TLS/SSL, HTTPS, PGP => RSA, ECC


use aes_gcm::{ aead::{Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm, Nonce, Key };


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path: String = String::from("./outenc/");
    let mut nombre: &str = "salida.enc";

    // Se genera la llave con valores random
    // un 
    let key = Aes256Gcm::generate_key(OsRng);

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, b"Mensaje Sin Cifrar".as_ref()).map_err(|e| format!("Decrypt error: {:?}", e))?;
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).map_err(|e| format!("Decrypt error: {:?}", e))?;

    println!("Key: {:?}", key);
    println!("Mensaje cifrado: {:?}", ciphertext);
    println!("Mensaje decifrado: {:?}", plaintext);


    assert_eq!(&plaintext, b"Mensaje Sin Cifrar");

    Ok(())
}
