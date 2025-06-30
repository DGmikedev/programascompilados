

pub struct Varilla{
    id:     usize,
    nombre: String,
    marca:  String,
    costo:  f32,
}

impl Varilla{
    pub fn new(_id:usize, _nombre:String, _marca:String, _costo:f32)->Varilla{
        let var = Varilla {   
                id : _id,
                nombre : _nombre,
                marca : _marca,
                costo : _costo,
        };
        var
    }

    pub fn varilla_grid(&self, ancho: f32, largo:f32, separacion: f32){
        let cnt_ancho = ancho / separacion;
        let cnt_largo = largo / separacion;
        println!("metraje de varilla: ancho = {}\nlargo = {}",cnt_ancho, cnt_largo );
    }

}

fn main() {
    let varilla = Varilla::new(1, "vrll_loza1".to_string(), "sanchez".to_string(), 120.0);
    
    varilla.varilla_grid(6.0, 9.0, 1.4);
}
