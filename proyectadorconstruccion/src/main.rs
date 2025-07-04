pub struct Escalera{
    altura: f32,
    longitud:  f32,
    huella: f32,
    chuella:f32
}

impl Escalera{
    pub fn new(_altura: f32 ,_longitud: f32 ,_huella: f32 ,_chuella:f32)->Escalera{
        let esc: Escalera = Escalera {   
                altura: _altura,
                longitud: _longitud,
                huella: _huella,
                chuella: _chuella,
        };
        esc
    }

    pub fn calula_esc(&self)->(f32, f32, f32, f32, f32, f32){
        let contra_huella: f32 = (self.altura / self.chuella).round();  // numero de contrahuellas
        let huella_esc: f32 = contra_huella - 1.0;                      // numero de huellas en escalera 
        let huellas_x_longitud: f32 = &huella_esc * self.huella;        // huellas * longitud para ver la longitud que se proyecta 
        let chuellas_x_altura: f32 =  &contra_huella * self.chuella;    // contrahuellas * contrahuella para ver la altura que se proyecta
        (self.altura, self.longitud, huella_esc, contra_huella, huellas_x_longitud, chuellas_x_altura)  
    }
}

fn main() {

    let escalera: (f32, f32, f32, f32, f32, f32) = 
        Escalera::new(
            1.275,
            1.98,
            0.28,
            0.17)
        .calula_esc();

    print!("\nAltura: {}\nLongitud: {}\n#Huellas: {}\n#Contra huellas: {}\nHuellas x Longitud: {}\nCHuellas x altura: {}", escalera.0,escalera.1,escalera.2,escalera.3, escalera.4, escalera.5);
}
