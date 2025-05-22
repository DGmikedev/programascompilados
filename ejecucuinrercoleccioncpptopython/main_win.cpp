#include <cstring>
#include <string>
#include <sstream>
#include <cstddef>
#include <iostream>
#include <cstdio>    // popen, pclose
#include <cstdlib>   // system()


bool consultaYejecutaPython(const std::string& comando, char* resultado, size_t max_len) {
    if (max_len > 0) { resultado[0] = '\0'; }

    FILE* pipe = _popen(comando.c_str(), "r");  // ← usar _popen en Windows
    if (!pipe) {
        const std::string error = "No se pudo ejecutar Python.";
        std::strncpy(resultado, error.c_str(), max_len - 1);
        resultado[max_len - 1] = '\0';
        return false;
    }

    char buffTemporal[256] = {0};
    size_t conteoLongitud = 0;

    while (fgets(buffTemporal, sizeof(buffTemporal), pipe) != NULL) {
        size_t longitudReal = strlen(buffTemporal);
        if (conteoLongitud + longitudReal < max_len) {
            std::strncat(resultado + conteoLongitud, buffTemporal, longitudReal);
            conteoLongitud += longitudReal;
        } else {
            size_t espacioTerm = max_len - 1 - conteoLongitud;
            std::strncat(resultado + conteoLongitud, buffTemporal, espacioTerm);
            conteoLongitud += espacioTerm;
            break;
        }
    }

    resultado[conteoLongitud] = '\0';

    int exit_code = _pclose(pipe);  // ← usar _pclose en Windows
    if (exit_code == -1) {
        const std::string str_error = "Error al cerrar el pipe Python.";
        std::strncpy(resultado, str_error.c_str(), max_len - 1);
        resultado[max_len - 1] = '\0';
        return false;
    }

    // En Windows, _pclose devuelve el código de salida directamente
    if (exit_code != 0) {
        std::string str_error = "El script Python terminó con error de salida: " + std::to_string(exit_code);
        std::strncpy(resultado, str_error.c_str(), max_len - 1);
        resultado[max_len - 1] = '\0';
        std::cerr << str_error << std::endl;
        return false;
    }

    return true;
}

std::string extraervalor(const std::string& json, const std::string& clave) {
    std::string buscando = "\"" + clave + "\": \"";
    size_t inciopos = json.find(buscando);
    if (inciopos == std::string::npos) {
        return "";
    }

    size_t ValorInicio = inciopos + buscando.length();
    size_t ValorFin = json.find("\"", ValorInicio);

    if (ValorFin == std::string::npos) {
        return "";
    }

    return json.substr(ValorInicio, ValorFin - ValorInicio);
}

int main() {
    char cTextR[300] = {0};
    std::string nombre;
    std::string host;
    std::string path;
    std::string comando = "python llaves.py";

    if (consultaYejecutaPython(comando, cTextR, sizeof(cTextR))) {

        std::string credencial(cTextR);

        nombre = extraervalor(credencial, "nombre");
        host = extraervalor(credencial, "host");
        path = extraervalor(credencial, "path");

        std::cout << "******************************" << std::endl;
        std::cout << "nombre: " << nombre << std::endl;
        std::cout << "host: " << host << std::endl;
        std::cout << "path: " << path << std::endl;
        std::cout << "******************************" << std::endl;

    } else {

        std::cerr << "Error al ejecutar la petición de credenciales" << std::endl;

    }

    system("pause");
    return 0;
}