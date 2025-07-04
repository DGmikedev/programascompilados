
#[derive(Debug)]
pub struct Proyecto{
    pub id: u16,
    pub nombre: String,
    pub departamento: String
}

#[derive(Debug)]
pub struct Usuario{
    pub id: u16,
    pub nombre_us: String,
    pub departamento_us: Vec<u16>
}

