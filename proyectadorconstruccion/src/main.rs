// DOSIFICACIÓN CEMENTO MOCTEZUMA
// https://www.academia.edu/37266141/TABLA_DOSIFICADORA_CPC_30_R



// RESISTENCIA DE 100Kg / cm2
// MUROS, PISOS, FIRMES, BANQUETAS
// 1 BLTO, 
// 2.5 BTS DE AGUA 
// 7 BTS DE ARENA
// 8 BTS DE GRABA
// 1 mt3 EL FACTOR ES X 5


// RESISTENCIA DE 150 Kg/cm2
// TRABES, CASTILLOS, DALAS, CADENAS
// 1 BLTO
// 2 BTS DE AGUA
// 5.5 BTS DE ARENA
// 6.5 BTS DE GRABA
// 1 mt3 EL FACTOR ES POR 6

// RESISTENCIA DE 200 Kg/cm2
// LOSAS, ENTREPISOS, TRABES, ZAPATAS
// 1 BLTO
// 7.75 BTS DE AGUA
// 4.5 BTS DE ARENA
// 5.5 BTS DE GRABA
// 1 mt3 EL FACTOR ES POR 7

// RESISTENCIA DE 250 Kg/cm2
// COLUMNAS Y LOSAS ESPECIALES
// 1 BLTO
// 1.5 BTS DE AGUA
// 3.5 BTS DE ARENA
// 5 BTS DE GRABA
// 1 mt3 EL FACTOR ES POR 8

// RESISTENCIA DE 300 Kg/cm2
// COLUMNAS Y LOSAS ESPECIALES
// 1 BLTO
// 1.25 BTS DE AGUA
// 2.5 BTS DE ARENA
// 4 BTS DE GRABA
// 1 mt3 EL FACTOR ES POR 9



#[derive(Debug)]
pub struct Losa{
    largo_mts:             f32,
    ancho_mts:             f32,
    alto_mts:              f32,
    cant_cemento_blt_50kg: f32,
    cant_arena_m3:         f32,
    cant_varilla_kg:       f32,
    cant_varilla_pza:      f32,
    cant_graba_m3:         f32,
    cant_agua_lts:         f32
}

impl Losa{

    pub fn new(_largo:f32, _ancho:f32, _alto:f32)->Losa{
        let losa: Losa = Losa {
            largo_mts : _largo,
            ancho_mts : _ancho,
            alto_mts  : _alto,
            cant_cemento_blt_50kg  : (_largo * _ancho * _alto ) * 7.5,          // numero de bultos de 50 Kg
            cant_varilla_kg : (_largo * _ancho) * 7.0,                          // Kilos de varilla    1 varilla de 3/8 = 6.6kg
            cant_varilla_pza:  ( (_largo * _ancho) * 7.0 ) / 6.6,               // varilla por pieza
            cant_arena_m3 : ( (_largo * _ancho * _alto ) * 7.5 ) * 4.0 * 0.02,  // m3 de arena
            cant_graba_m3 : ( (_largo * _ancho * _alto ) * 7.5 ) * 5.0 * 0.02,  // m3 de graba
            cant_agua_lts :  ( (_largo * _ancho * _alto ) * 7.5 ) * 40.0        // Lts de agua
        };
        losa
    }

    pub fn tolerancias(self, tol: f32)-> Vec<f32>{
        let tol_cant_cemento_blt_50kg =  self.cant_cemento_blt_50kg +  ( (self.cant_cemento_blt_50kg * tol ) / 100.0);
        let tol_cant_varilla_kg =        self.cant_varilla_kg +  ( (self.cant_varilla_kg * tol ) / 100.0);
        let tol_cant_varilla_pza =       self.cant_varilla_pza +  ( (self.cant_varilla_pza * tol ) / 100.0);
        let tol_cant_arena_m3 =          self.cant_arena_m3 +  ( (self.cant_arena_m3 * tol ) / 100.0);
        let tol_cant_graba_m3 =          self.cant_graba_m3 +  ( (self.cant_graba_m3 * tol ) / 100.0);
        let tol_cant_agua_lts =          self.cant_agua_lts +  ( (self.cant_agua_lts * tol ) / 100.0);

        let res: Vec<f32> = vec![
            tol_cant_cemento_blt_50kg ,
            tol_cant_varilla_kg ,
            tol_cant_varilla_pza ,
            tol_cant_arena_m3 ,
            tol_cant_graba_m3 ,
            tol_cant_agua_lts ]
        ;
        res
    }
}

fn main(){
    let losa = Losa::new(20.0,10.0,0.10);
    println!("{:?}",losa);

    println!("{:?}", losa.tolerancias(5.0));
}


/*
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

*/