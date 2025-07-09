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
id_dos	marca	resistencia	bultos	kg por bulto	botes de agua	lts_agua	kgs_agua	mts3_agua	botes de arena	mts3_arena	botes de grava	mts3_grava	factorXmt3	
1	moctezuma	100 kg/cm²	1	50	2.5	47.5	47.5	0.0475	7	0.133	8	0.152	5	
2	moctezuma	150 kg/cm²	1	50	2	38	38	0.038	5.5	0.1045	6.5	0.1235	6	
3	moctezuma	200 kg/cm²	1	50	1.75	33.25	33.25	0.03325	4.5	0.0855	5.5	0.1045	7	
4	moctezuma	250 kg/cm²	1	50	1.5	28.5	28.5	0.0285	3.5	0.0665	5	0.095	8	
5	moctezuma	300 kg/cm²	1	50	1.25	23.75	23.75	0.02375	2.5	0.0475	4	0.076	9	
6	CruzAzul_tipoII CPC 30R RS	100 kg/cm²	1	50	2.5	45	45	0.045	6.25	0.1125	7.25	0.1305	5	Se consideran botes de 18 Lts para revenimiento de 12-14 cm
7	CruzAzul_tipoII CPC 30R RS	150 kg/cm²	1	50	2.25	40.5	40.5	0.0405	5.5	0.099	6.75	0.1215	6	Se consideran botes de 18 Lts para revenimiento de 12-14 cm
8	CruzAzul_tipoII CPC 30R RS	200 kg/cm²	1	50	1.75	31.5	31.5	0.0315	4.25	0.0765	5.25	0.0945	7	Se consideran botes de 18 Lts para revenimiento de 12-14 cm
9	CruzAzul_tipoII CPC 30R RS	250 kg/cm²	1	50	1.5	27	27	0.027	3.5	0.063	4.5	0.081	8	Se consideran botes de 18 Lts para revenimiento de 12-14 cm
10	CruzAzul_tipoII CPO 40B	100kg/cm2	2	25	2.5	45	45	0.045	7.75	0.1395	8	0.144	5	Se consideran botes de 18 Lts para revenimiento de 12-14 cm
11	CruzAzul_tipoII CPO 40B	150kg/cm2	2	25	2.5	45	45	0.045	6.75	0.1215	7.25	0.1305	6	Se consideran botes de 18 Lts para revenimiento de 12-14 cm
12	CruzAzul_tipoII CPO 40B	200kg/cm2	2	25	2	36	36	0.036	5.75	0.1035	6.75	0.1215	7	Se consideran botes de 18 Lts para revenimiento de 12-14 cm
13	CruzAzul_tipoII CPO 40B	250kg/cm2	2	25	1.75	31.5	31.5	0.0315	4.25	0.0765	5.5	0.099	8	Se consideran botes de 18 Lts para revenimiento de 12-14 cm
14	CruzAzul_tipoII CPO 40B	300kg/cm2	2	25	1.75	31.5	31.5	0.0315	3.5	0.063	4.5	0.081	5	Se consideran botes de 18 Lts para revenimiento de 12-14 cm
15	CruzAzul_Mortero	N/A	1	50	N/A	N/A	N/A	N/A	2.5	0.045	4.5	0.081	N/A	Dosificación: Únicamente adiciona arena y agua en proporciones adecuadas.
16	CruzAzul_Mortero	N/A	1	50	N/A	N/A	N/A	N/A	2	0.036	4	0.072	N/A	Dosificación: Únicamente adiciona arena y agua en proporciones adecuadas.
17	CruzAzul_Mortero	N/A	1	50	N/A	N/A	N/A	N/A	6	0.108	N/A	N/A	N/A	Dosificación: Únicamente adiciona arena y agua en proporciones adecuadas.
18	CruzAzul_Mortero	N/A	1	50	N/A	N/A	N/A	N/A	7	0.126	N/A	N/A	N/A	Dosificación: Únicamente adiciona arena y agua en proporciones adecuadas.
19	CruzAzul_Mortero	N/A	1	50	N/A	N/A	N/A	N/A	5	0.09	N/A	N/A	N/A	Dosificación: Únicamente adiciona arena y agua en proporciones adecuadas.
20	CruzAzul_Mortero	N/A	1	50	N/A	N/A	N/A	N/A	2	0.036	N/A	N/A	N/A	Dosificación: Únicamente adiciona arena y agua en proporciones adecuadas.
21	Moctezuma CPC 30 R	100 kg/cm2	1	50	2.5	45	45	0.045	7	0.126	8	0.144	5	Se consideran botes de 18 Lts y resistencia con grava 3/4" 
22	Moctezuma CPC 30 R	150 kg/cm2	1	50	2	36	36	0.036	5.5	0.099	6.5	0.117	6	Se consideran botes de 18 Lts y resistencia con grava 3/4" 
23	Moctezuma CPC 30 R	200 kg/cm2	1	50	1.75	31.5	31.5	0.0315	4.5	0.081	5.5	0.099	7	Se consideran botes de 18 Lts y resistencia con grava 3/4" 
24	Moctezuma CPC 30 R	250 kg/cm2	1	50	1.5	27	27	0.027	3.5	0.063	5	0.09	8	Se consideran botes de 18 Lts y resistencia con grava 3/4" 
25	Moctezuma CPC 30 R	300 kg/cm2	1	50	1.25	22.5	22.5	0.0225	3	0.054	4	0.072	9	Se consideran botes de 18 Lts y resistencia con grava 3/4" 
26	Moctezuma CPC 30 R RS	100 kg/cm2	1	50	2.5	45	45	0.045	7	0.126	8	0.144	5	Se consideran botes de 18 Lts y resistencia con grava 3/4" 
27	Moctezuma CPC 30 R RS	150 kg/cm2	1	50	2	36	36	0.036	5.5	0.099	6.5	0.117	6	Se consideran botes de 18 Lts y resistencia con grava 3/4" 
28	Moctezuma CPC 30 R RS	200 kg/cm2	1	50	1.75	31.5	31.5	0.0315	4.5	0.081	5.5	0.099	7	Se consideran botes de 18 Lts y resistencia con grava 3/4" 
29	Moctezuma CPC 30 R RS	250 kg/cm2	1	50	1.5	27	27	0.027	3.5	0.063	5	0.09	8	Se consideran botes de 18 Lts y resistencia con grava 3/4" 
30	Moctezuma CPC 30 R RS	300 kg/cm2	1	50	1.25	22.5	22.5	0.0225	3	0.054	4	0.072	9	Se consideran botes de 18 Lts y resistencia con grava 3/4" 
31	Cemex Extra Tolteca CPC 30 R	100 kg/cm2	1	50	3	57	57	0.057	8	0.152	8.5	0.1615	5	Se consideran botes de 19
32	Cemex Extra Tolteca CPC 30 R	150 kg/cm2	1	50	2.5	47.5	47.5	0.0475	5.5	0.1045	6.5	0.1235	6	Se consideran botes de 19
33	Cemex Extra Tolteca CPC 30 R	200 kg/cm2	1	50	2	38	38	0.038	4	0.076	6	0.114	7	Se consideran botes de 19
34	Cemex Extra Tolteca CPC 30 R	250 kg/cm2	1	50	1.5	28.5	28.5	0.0285	3.5	0.0665	5	0.095	8	Se consideran botes de 19
35	Cemex Extra Tolteca CPC 30 R	300 kg/cm2		50	1.5	28.5	28.5	0.0285	2.5	0.0475	4.5	0.0855	9	Se consideran botes de 19
36	Fortaleza CPC30 R	100 kg/cm2	1	50										
37	Fortaleza CPC30 R	150 kg/cm2	1	50										
38	Fortaleza CPC30 R	200 kg/cm2	1	50	2				4.5		6.5		7	
39	Fortaleza CPC30 R	250 kg/cm2	1	50										
40	Fortaleza CPC30 R	300 kg/cm2	1	50										


*/
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