# CREADOR DE LOG CON DATOS OFUSCADOS O ENMASCARADOS

Este programa es un prototipo de funciones que se usarón para crear<br>
uin log ofuscado el cual ofusca o enmascara los datos <br>
que no se desea que sean vistos por personas no autorizadas para ello.

![AQUÍ LA CARPETA DEL PROYECTO](logofuscado)

## COMPILAR EN LA CARPETA logofuscado

```bash
g++ maincrearlog.cpp ofuscador.cpp -o rmcrearlogofuscado.exe
```

## CONTIENE DOS FUCNONES

### ofuscador

```cpp
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
void ofuscador(char* cadena, std::string opcion)
```

### maincrearog

```cpp
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
bool crearlog(char* nombre, char* mensaje)
```

un extracto de ejemplo el cual es producto de estas funciones:

```txt
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,LOPE80010*********)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,GARC92031*********)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,HERN99061*********)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,MART85072*********)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,RAMI01010*********)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,SANC78022*********)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,REYE95091*********)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,CAST62050*********)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,TORR04123*********)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,DELA87111*********)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,******************)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,******************)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,******************)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,******************)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,******************)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,******************)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,******************)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,******************)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,******************)
[20250618][maincrearlog.cpp][9804]EXECUTE fn_insertcurp(20:08:48,******************)
```