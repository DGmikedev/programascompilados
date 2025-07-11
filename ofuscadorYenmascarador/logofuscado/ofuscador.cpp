#include <ostream>  // std::string,  printf
#include <cstring>  // strlen, memset


/**
 *  Cambia la mitad o el total de los caracteres de un buffer
 *  si cambia toda la cadena del buffer enviado a esto le llama (la función)
 *  ofuscar y cambia la mitad de caracteres a esto la función le llama enmascarar
 *  
 *  ptr cadena    buffer a tratar
 *  string opcion  "OFUSCAR" , "ENMASCARAR" 
 *  
 * return void
 * 
 */
void ofuscador(char* cadena, std::string opcion){

    size_t longitud = strlen(cadena);
    size_t set_lng = 0;


    if(opcion == "OFUSCAR")
    {
        memset(cadena, '*' , longitud);
        return;
    }
    else if(opcion == "ENMASCARAR")
    {

        // se revisa la longtud 
        // del la traza a enmascarar
        // si es par solo se ofusca la mitad del mensaje
        if((longitud%2) == 0){ set_lng = longitud / 2 ;  
        
        // si la longitu no es par se ofusca la mitad + 1
        // caracteres del mensaje
        }
        else
        { 
            (set_lng = longitud / 2) + 1 ; 
        }

        for( int i = set_lng; i < longitud; i++)
        {
            cadena[i] = '*';
        }
        return;
    }
    else
    {
        // si no coincide la opción deseada
        snprintf(cadena, sizeof(cadena), "no_ofuscada");
        return;
    }

}