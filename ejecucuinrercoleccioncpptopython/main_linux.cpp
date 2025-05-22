/// librerias remediacines /////////////
#include <cstring>
#include <string>  
#include <sstream>
#include <cstddef> // Necesario para size_t
#include <iostream>  // Para std::cout, std::cerr
#include <cstdio>    // Para popen, pclose, fgets
// #include <sys/wait.h> 
////////////////////////////////////////


bool getCredentials(const std::string& comando, char* resultado, size_t max_len){

    if (max_len > 0){ resultado[0] = '\0'; } // resltado vacio al iniciar

    FILE* pipe = popen(comando.c_str(), "r");
    if(!pipe){
        const std::string error = "No se pudo ejecutar python";
        std::strncpy(resultado, error.c_str(), max_len - 1);
        resultado[max_len - 1] = '\0'; // terminación nula 
        return false;
    }
    
    char buffTemporal[256] = {0};
    size_t conteoLongitud = 0;
    
    // loop hasta que pipe ya no tenga datos o resultado se llene
    while(fgets(buffTemporal, sizeof(buffTemporal), pipe) != NULL){
        
        size_t longitudReal = strlen(buffTemporal);

        // se verifica que no se exceda el tamaño del buffer 
        if(conteoLongitud + longitudReal < max_len){
            std::strncat(resultado + conteoLongitud, buffTemporal, longitudReal);
            conteoLongitud += longitudReal;
        }else{
            size_t espacioTermiancion = max_len - 1 - conteoLongitud;
            std::strncat(resultado + conteoLongitud, buffTemporal,espacioTermiancion );
            conteoLongitud +=espacioTermiancion;
            break;
        }
    }

    resultado[conteoLongitud] = '\0';

    int status = pclose(pipe);
    if(status == -1){
        const std::string str_error = "Error al cerrar el pipe Python";
        std::strncpy(resultado, str_error.c_str(), max_len - 1);
        resultado[max_len - 1] = '\0';
        return false; 
    }else{

        // Verifica si el proceso del python termino bien!
        if (WIFEXITED(status)) {
            int exit_status = WEXITSTATUS(status);
            if (exit_status != 0) {
                // El script Python terminó con un código de error (distinto de 0)
                const std::string str_error = "El script Python terminó con error de salida: "; //  + std::stringstream(exit_status);
                std::strncpy(resultado, str_error.c_str(), max_len - 1); // Sobrescribe con error
                resultado[max_len - 1] = '\0';
                std::cerr << str_error << std::endl;
                return false;
            }
        } else {
            
            const std::string str_error = "El script Python no terminó normalmente.";
            std::strncpy(resultado, str_error.c_str(), max_len - 1);
            resultado[max_len - 1] = '\0';
            std::cerr << str_error << std::endl;
            return false;
        }

    }

    return true;
}

std::string extraervalor(const std::string& json, const std::string& clave){

    // revisar que la regex sea exacta
    std::string buscando = "\"" + clave + "\": \"";
    
    size_t inciopos = json.find(buscando);
    if(inciopos == std::string::npos){
        return "";
    }

    size_t ValorInicio = inciopos + buscando.length();
    size_t ValorFin = json.find("\"", ValorInicio);

    if(ValorFin == std::string::npos){
        return "";
    }
    return json.substr(ValorInicio, ValorFin - ValorInicio);
}



int main(){


    char cTextR[300]={0}; 
    std::string host;
    std::string comando = "python llaves.py";
    
    if(getCredentials(comando, cTextR, sizeof(cTextR) ))
    {
        std::string credencial(cTextR);
        host = extraervalor(credencial, "host");  // Aqui obtenemos el producto final que es el dato entre las etiquetas
    }else{
        printf("Error al ejecutar la petición de credenciales");
    }

    system("pause");
    return 0;
}