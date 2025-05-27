use umya_spreadsheet::*;

fn main() {
   
    let mut book = new_file();

    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();

    // Crear un nuevo estilo
    let mut style = Style::default();
    let mut style2 = Style::default();

    // Configura alineación
    let mut alignment = Alignment::default();
    alignment.set_horizontal(HorizontalAlignmentValues::Center);
    alignment.set_vertical(VerticalAlignmentValues::Center);
    alignment.set_wrap_text(true);
    style.set_alignment(alignment.clone());

    style2.set_alignment(alignment.clone());
    
    // Configura fuente
    let mut font = style.get_font_mut();
    font.set_bold(true);

    let mut font2 = style2.get_font_mut();
    font2.set_bold(true);


    // Configura bordes
    style.get_borders_mut().get_bottom_mut().set_border_style(Border::BORDER_THIN);
    style.get_borders_mut().get_right_mut().set_border_style(Border::BORDER_THIN);
    style.get_borders_mut().get_top_mut().set_border_style(Border::BORDER_THIN);

    // Configura bordes
    style2.get_borders_mut().get_bottom_mut().set_border_style(Border::BORDER_THIN);
    style2.get_borders_mut().get_right_mut().set_border_style(Border::BORDER_THIN);
    style2.get_borders_mut().get_top_mut().set_border_style(Border::BORDER_THIN);
   

    // Confifura color de fondo
    style.set_background_color(Color::COLOR_RED);

    sheet.set_name("aviso");
    sheet.add_merge_cells("A1:D2");
    sheet.get_cell_mut("A1").set_value("LOG CREDENCIALES NO MODIFICAR ")
        .set_style(style.clone());

    sheet.add_merge_cells("A3:D12");
    sheet.get_cell_mut("A3").set_value("Precaución: Este documento contiene información sensible y confidencial. Su manejo indebido puede comprometer la privacidad de personas o entidades. No lo compartas, almacenes ni transmitas por canales no seguros. Asegúrate de cumplir con las políticas de seguridad y protección de datos aplicables.")
        .set_style(style.clone());
    
    sheet.add_merge_cells("A13:D16");
    sheet.get_cell_mut("A13").set_value("CARGUE EN SISTEMA Y DESTRUYA, NO COPIE, NO ENCRIPTE, NO APUNTE; LA SEGURIDAD DEPENDE DE TODOS.")
        .set_style(style.clone());

    sheet.get_cell_mut("E1").set_value("fecha_solicitud");
    sheet.get_cell_mut("F1").set_value("01-05-2025");
    sheet.get_cell_mut("G1").set_value("ip_target");
    sheet.get_cell_mut("H1").set_value("198.162.254.198");

    sheet.get_cell_mut("E2").set_value("rfc_servicio");
    sheet.get_cell_mut("F2").set_value("12584");
    sheet.get_cell_mut("G2").set_value("id");
    sheet.get_cell_mut("H2").set_value("serv_terr_test");

    sheet.get_cell_mut("E3").set_value("departamento");
    sheet.get_cell_mut("F3").set_value("Bnc_25");
    sheet.get_cell_mut("G3").set_value("path_enc");
    sheet.get_cell_mut("H3").set_value("../path/cred");

    sheet.get_cell_mut("E4").set_value("fec_ini_atnc");
    sheet.get_cell_mut("F4").set_value("01-07-2025");
    sheet.get_cell_mut("G4").set_value("path_doc");
    sheet.get_cell_mut("H4").set_value("../path/doc");

    sheet.get_cell_mut("E5").set_value("fec_ini_atnc");
    sheet.get_cell_mut("F5").set_value("01-07-2025");
    sheet.get_cell_mut("G5").set_value("path_doc");
    sheet.get_cell_mut("H5").set_value("../path/doc");

    sheet.get_cell_mut("E6").set_value("fec_fin_atnc");
    sheet.get_cell_mut("F6").set_value("03-07-2025");
    sheet.get_cell_mut("G6").set_value("fec_in_serv");
    sheet.get_cell_mut("H6").set_value("05-07-2025");

    sheet.get_cell_mut("E7").set_value("emp_solicito");
    sheet.get_cell_mut("F7").set_value("26584");
    sheet.get_cell_mut("G7").set_value("sello_linux");
    sheet.get_cell_mut("H7").set_value("c2VsbG9sW5eDE1LzIwMjQvMQ==");

    sheet.get_cell_mut("E8").set_value("empl_id_geren");
    sheet.get_cell_mut("F8").set_value("125");
    sheet.get_cell_mut("G8").set_value("sello_ger");
    sheet.get_cell_mut("H8").set_value("c2Vsb9nZXJlbmNpYWFmbJlMTUvMjAyNC8x");

    sheet.add_merge_cells("E9:H9");
    sheet.get_cell_mut("E9").set_value("RESPONSABLES").set_style(style2.clone());

    sheet.get_cell_mut("E10").set_value("guardado");
    sheet.get_cell_mut("E11").set_value("observador");
    sheet.get_cell_mut("E12").set_value("técnico");
    sheet.get_cell_mut("E13").set_value("gerent_solicito");

    sheet.get_cell_mut("F10").set_value("###2365984");
    sheet.get_cell_mut("F11").set_value("####232514");
    sheet.get_cell_mut("F12").set_value("####265948");
    sheet.get_cell_mut("F13").set_value("###2659487");

    sheet.get_cell_mut("G10").set_value("area_num");
    sheet.get_cell_mut("G11").set_value("area_num");
    sheet.get_cell_mut("G12").set_value("area_num");
    sheet.get_cell_mut("G13").set_value("area_num");

    sheet.get_cell_mut("H10").set_value("area_solicitante");
    sheet.get_cell_mut("H11").set_value("seg_sefvicios");
    sheet.get_cell_mut("H12").set_value("servicios");
    sheet.get_cell_mut("H13").set_value("area_solicitante");

    sheet.add_merge_cells("E14:H16");

    // writer
    let path = std::path::Path::new("logcredenciales.xlsx");
    let _ = writer::xlsx::write(&book, path);
}
