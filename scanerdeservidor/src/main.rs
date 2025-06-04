
use std::process::Command;
use std::fs::File;
use std::io::Write;
use chrono::prelude::*;


/*
[Site]-[Room]-[Row]-[Rack]-[Unit]
DC1   - R01  - B04 - RK04 - U17

srv-web01	Dell R740	192.168.1.10	R01-B04-RK04-U17	Servidor web principal
srv-db01	HP DL380	192.168.1.20	R01-B05-RK02-U10	Base de datos

+------------------------------+
¦     ?? SERVIDOR FÍSICO       ¦
¦------------------------------¦
¦ Hostname:   srv-web01        ¦
¦ Modelo:     Dell R740        ¦
¦ IP:         192.168.1.10     ¦
¦----------------------------  ¦
¦ Ubicación:                   ¦
¦ Sala:       Room 01          ¦
¦ Fila:       B                ¦
¦ Rack:       04               ¦
¦ Unidad:     U17              ¦
¦ Código:     R01-B04-RK04-U17 ¦
+-------------------------------+


BDts
sqlcmd -S localhost -E -Q "SELECT @@VERSION"
-S: el nombre de la instancia (usa localhost si es por defecto).
-E: autenticación integrada (Windows).
-Q: ejecuta la consulta y termina.

Import-Module SqlServer
$server = New-Object Microsoft.SqlServer.Management.Smo.Server "localhost"
$server.Information | Select Edition, Version, ProductLevel, EngineEdition


mysql -u root -p -e "SELECT VERSION(), @@version_comment;"
mysql -u root -p -e "SHOW VARIABLES LIKE 'port';"
mysql -u root -p -e "SHOW VARIABLES LIKE '%dir%';"                       Para ver solo información clave (por ejemplo, datos de memoria y paths):
mysql -u root -p -e "SHOW VARIABLES LIKE 'innodb_buffer_pool_size';"    Para ver solo información clave (por ejemplo, datos de memoria y paths):
mysql -u root -p -e "SHOW ENGINES;"                                Mostrar todos los motores de almacenamiento soportados
mysql -u root -p -e "SHOW VARIABLES LIKE 'bind_address';"          Ver configuración de red (puerto, bind address)
mysql -u root -p -e "SELECT user, host, plugin FROM mysql.user;"    Ver usuarios, plugins y autenticación 
mysqladmin -u root -p status                                        Usar status (resumen rápido del servidor)



postgreSQL

psql -U postgres -c "SELECT version();"
psql -U postgres -c "SHOW ALL;"
psql -U postgres -c "SHOW config_file;"
psql -U postgres -c "SHOW data_directory;"
psql -U postgres -c "SHOW hba_file;"

psql -U postgres -c "\l"                               Bases de datos:
psql -U postgres -c "\du"                              Roles:
psql -U postgres -c "SELECT * FROM pg_stat_activity;"  Conexiones activas:
psql -U postgres -c "SELECT * FROM pg_extension;"      módulos/extensiones instaladas








*/

/** 
*  Funció que obtiene el sistema operativo en que 
*  corre el programa linux o windows
*  return i8 , 0 = windows, 1 = linux
*/
fn get_info_system()->u8{
    let opc_so: bool = cfg!(target_os = "windows");
    if opc_so { return 0u8 }
    1u8 
}

fn ejecutar_comando(comando:&str, args: &[String])->Vec<u8>{

    let output = Command::new(comando)
        .args(args)
        .output()
        .expect(" ERROR : [ Comando no ejecutado ]");

    return output.stdout
}


fn obtener_memoria_windows()->Vec<u8>{
    // array de argumetos
    let args: [String; 2] = [
        "/C".to_string(), 
        "wmic MemoryChip get BankLabel, Capacity, MemoryType, Speed, Manufacturer".to_string()
    ];

    // comando en wnodws
    let comando = "cmd";

    return ejecutar_comando(comando, &args);
}



/**
 * Fucnion que retorna strings para formar el texto
 * del informe
 * return String 
*/
fn mensages(opc:usize)->String{
    let lista_mensages = [
        "-----------------------------------------------\n".to_string(),
        "\n".to_string(),
        "HERRAMIENTA PARA ESCANEO DE PAQUETES EN SERVIDOR\n".to_string(),
        "\nDONDE:\nLabel: Ranura donde se encuentra la RAM.\nCapacity: Capacidad de la memoria (expresada en bytes).\nManufacturer: Fabricante de la memoria.\nMemoryType: Tipos de memoria RAM donde 20=DDR, 21=DD2R=2, 24=DDR3.\nSpeed: Velocidad de respuesta de cada RAM (expresado en MHZ).\n".to_string(),
        "WINDOWS".to_string(),
        "LINUX".to_string(),
    ];
    let ret = lista_mensages[opc].clone();
    return ret;
}


fn main() {
    
    // string de texto
    let mut texto: String = String::new();


    // se incluye linea de texto incial
    texto.push_str(&mensages(2));
    
    // se guada separador
    texto.push_str(&mensages(0));

    // se consulta fecha y hora
    let utc: DateTime<Utc> = Utc::now(); 
    let fecha = format!("\n{}\n\n", utc.format("FECHA: %Y-%m-%d - HORA: %H:%M:%S"));
    texto.push_str(&fecha);


    // tratamiento de etiqueta ////////////////////////////////////////////////////////////

    let tag = "SERVIDOR: R01-B04-RK04-U17\n";

    // se guada naombre de servidor
    texto.push_str(tag);

        // se guada salto de linea
    texto.push_str(&mensages(1));

    // se guada separador
    texto.push_str(&mensages(0));
    
    // tratamiento de etiqueta ////////////////////////////////////////////////////////////



    // se revisa si esta corriendo
    // windows o linux
    let so = get_info_system();

    if so == 0u8 {  

        // se guada salto de linea
        texto.push_str(&mensages(1));

        texto.push_str( &format!("SISTEMA OPERATIVO: [{}]\n" ,mensages(4)) );

        // se guada salto de linea
        texto.push_str(&mensages(1));

        // se obtiene caracteristicas de tarjetas de memoria
        texto.push_str(  &String::from_utf8_lossy(&obtener_memoria_windows()) );

        // se guada informacion de caravateristicas windows
        texto.push_str(&mensages(3));

        // // se guada salto de línea
        // texto.push_str(&mensages(1));

        // se guada separador
        texto.push_str(&mensages(0));

    }else{  
         texto.push_str( &format!("SISTEMA OPERATIVO: [{}]\n" ,mensages(4)) );
    }

     
    //




    let mut archivo = File::create("snapshot.txt").expect("ERROR: [No fue posible crear el archivo]");
    archivo.write_all(&texto.as_bytes()).expect("ERROR: [NO SE PUDO ESCRIBIR EN ARCHIVO CREADO]");


   // obtener_memoria_windows();





/*


    Label: Ranura donde se encuentra la RAM.
    Capacity: Capacidad de la memoria (expresada en bytes).
    Manufacturer: Fabricante de la memoria.
    MemoryType: Tipos de memoria RAM donde 20=DDR, 21=DD2R=2, 24=DDR3.
    Speed: Velocidad de respuesta de cada RAM (expresado en MHZ).


    "aix", "android", "cuda", "dragonfly", "emscripten", "espidf", 
    "freebsd", "fuchsia", "haiku", "hermit", "horizon", "hurd", 
    "illumos", "ios", "l4re", "linux", "macos", "netbsd","none",
    "nto", "nuttx", "openbsd","psp", "psx", "redox", "rtems", "solaris",
    "solid_asp3", "teeos", "trusty", "tvos", "uefi", "unknown", "visionos","vita" 
*/

 /*

    let output = if cfg!(target_os = "windows"){
        Command::new("cmd")
        .args(["/C", "dir"])
        .output()
        .expect(" ERROR : [ Comando no ejecutado ]")
    }else{
        Command::new("sh")
        .args(["-c", "ls"])
        .output()
        .expect(" ERROR : [ Comando no ejecutado ]")
    };

    

    let txt: &Vec<u8> = &output.stdout;// String::from_utf8_lossy(&output.stdout);

    let mut archivo = File::create("snapshot.txt")
                .expect("ERROR: [No fue posible crear el archivo]");

    archivo.write_all(&txt).expect("ERROR: [NO SE PUDO ESCRIBIR EN ARCHIVO CREADO]");

    
  */
    
    
    
    //.expect("programm output was not valid utf-8"));
    // let salida = String::from_utf8(output.stdout).expect("programm output was not valid utf-8");
    // println!("{:?}", salida);
    // println!("{:?}",salida);
    // println!("{:?}", salida);
}
