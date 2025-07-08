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

/*
Tabla de conversión: m³, kg y L
Material	        Densidad (kg/m³)	Peso por 1 m³ (kg)	Peso por 1 litro (kg/L)
Agua	            1,000	            1,000 kg	        1.00 kg/L
Arena seca	        1,500 – 1,600	    1,500 – 1,600 kg	1.50 – 1.60 kg/L
Arena húmeda	    1,800 – 2,000	    1,800 – 2,000 kg	1.80 – 2.00 kg/L
Grava	            1,600 – 1,800	    1,600 – 1,800 kg	1.60 – 1.80 kg/L
Cemento (a granel)	~1,400	            ~1,400 kg	        ~1.40 kg/L
Concreto fresco	    2,300 – 2,500	    2,300 – 2,500 kg	2.30 – 2.50 kg/L
Tierra suelta	    1,200 – 1,400	    1,200 – 1,400 kg	1.20 – 1.40 kg/L
*/



// bote de agua * 20 = lts
// bote arena * 52 = 1 mt3

/*
Paso 1: Conocer la densidad de la grava

La densidad de la grava (seca y limpia) varía, pero en promedio es:

    1,500 a 1,800 kg/m³

Vamos a usar un valor intermedio:

    1,600 kg/m³ (es decir, 1.6 kg por litro)

✅ Paso 2: Convertir litros a metros cúbicos

Sabemos que:

    1 m³ = 1,000 litros

    Entonces, 20 litros = 0.02 m³

    */









pub struct Dosificacion{

}