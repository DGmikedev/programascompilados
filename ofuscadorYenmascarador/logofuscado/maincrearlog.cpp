#include <ostream>  // para std::string, printf
#include <fstream>  // manejo de archivos
#include <iostream> // para  std::ios::app append 
#include <chrono>   // time
#include <iomanip>  // put_time



// del archivo ofuscador.cpp
void ofuscador(char*, std::string);


/**
 *  Crea y scribe una línea de texto
 *  en un archivo .txt que funciona como log
 * 
 *  char* nombre  es el nombre del archivo a crear o abrir 
 *  char* mensaje es el texto que se adjuntará el archivo de texto
 * 
 *  return boolean  si cumple con las funciones la salida será true
 *                  de caso contrarío sera false
 */
bool crearlog(char* nombre, char* mensaje){


    // buffer para albergar el mensaje
    char _mensaje[200];
    
    // buffer para el nombre del archivo
    char _nombrecompleto[50]; 
    
    
    _nombrecompleto[0] = '\0';
    _mensaje[0] = '\0';
    
    // obtener datos de tiempo
    std::ofstream fs;               // estacia del manejador de archivos
    time_t tTiempo;                 // dato de tiempo
    struct tm *tHora;               // estructura para varias variable
    tTiempo = time(NULL);           // con NULL devuelve el tiempo 
    tHora = localtime(&tTiempo);    // valor se tiempo a estructura 


    // se ajusta el nombre
    snprintf(_nombrecompleto, sizeof(_nombrecompleto), "%s%02i%02i.txt", nombre, tHora->tm_mon + 1, tHora->tm_mday);

    // se adjunta el mensaje
    snprintf(_mensaje, sizeof(_mensaje), "[%04d%02d%02d][%s][%i]%s",tHora->tm_year + 1900,tHora->tm_mon + 1,tHora->tm_mday,__FILE__,getpid(),mensaje);


    // se abre o se crea el archivo 
    fs.open(_nombrecompleto, std::ios::app);

    if( fs.is_open() )
    {
        fs << _mensaje;
        fs << "\n";
        fs.close();
    }
    else
    {
        return false;
    }
    
    return true;
}


int main(){

    char nombre[100];
    char mensaje[300];
    char curpofuscada[19];

    nombre[0] = '\0';
    mensaje[0] = '\0';
    curpofuscada[0] = '\0';

    char curps[10][19] = {
        "LOPE800101HDFRNL09",
        "GARC920315MCMRRN03",
        "HERN990612HDFLSL07",
        "MART850724MNLPPD01",
        "RAMI010101HGTZMN00",
        "SANC780223MDFCRR06",
        "REYE950911HCMGLS04",
        "CAST620507MDFRBN02",
        "TORR041230HNLVRL08",
        "DELA871110MDFGRC05"
    };

    snprintf(nombre, sizeof(nombre), "insertador_curps");

    for(int i = 0; i < 10; i++){
        
        // mensaje consola
        std::cout <<"CURP INSERTADA: " << curps[i] << std::endl;

        snprintf(curpofuscada, sizeof(curpofuscada), curps[i]);

        ofuscador(curpofuscada, "ENMASCARAR");
        
        // ajusta mensaje a inserta en log
        snprintf(mensaje, sizeof(mensaje), "EXECUTE fn_insertcurp(%s,%s)",__TIME__, curpofuscada);

        // se manda a log
        crearlog(nombre, mensaje);

        // se limia el buffer
        mensaje[0] = '\0';

        // se limpia el buffer de la curp
        curpofuscada[0] = '\0';
    
    }

    for(int i = 0; i < 10; i++){
        
        // mensaje consola
        std::cout <<"CURP INSERTADA: " << curps[i] << std::endl;

        snprintf(curpofuscada, sizeof(curpofuscada), curps[i]);

        ofuscador(curpofuscada, "OFUSCAR");
        
        // ajusta mensaje a inserta en log
        snprintf(mensaje, sizeof(mensaje), "EXECUTE fn_insertcurp(%s,%s)",__TIME__, curpofuscada);

        // se manda a log
        crearlog(nombre, mensaje);

        // se limia el buffer
        mensaje[0] = '\0';

        // se limpia el buffer de la curp
        curpofuscada[0] = '\0';
    
    }

}