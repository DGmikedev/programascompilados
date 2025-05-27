use umya_spreadsheet::*;

fn main() {
   
    let mut book = new_file();

    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();

    // Crear un nuevo estilo
    let mut style = Style::default();

    // Configura alineación
    let mut alignment = Alignment::default();
    alignment.set_horizontal(HorizontalAlignmentValues::Center);
    alignment.set_vertical(VerticalAlignmentValues::Center);
    alignment.set_wrap_text(true);
    style.set_alignment(alignment);
    
    // Configura fuente
    let mut font = style.get_font_mut();
    font.set_bold(true);

    // Configura bordes
    style.get_borders_mut().get_bottom_mut().set_border_style(Border::BORDER_MEDIUM);
    style.get_borders_mut().get_left_mut().set_border_style(Border::BORDER_MEDIUM);
    style.get_borders_mut().get_right_mut().set_border_style(Border::BORDER_MEDIUM);

    // Confifura color de fondo
    let mut color = Color::default();
    color.set_argb("F34F8080");
    style.set_background_color(color);

    sheet.set_name("aviso");
    sheet.add_merge_cells("A1:D2");
    sheet.get_cell_mut("A1").set_value("LOG CREDENCIALES NO MODIFICAR ")
        .set_style(style.clone());

    sheet.add_merge_cells("A3:D12");
    sheet.get_cell_mut("A3").set_value("Precaución: Este documento contiene información sensible y confidencial. Su manejo indebido puede comprometer la privacidad de personas o entidades. No lo compartas, almacenes ni transmitas por canales no seguros. Asegúrate de cumplir con las políticas de seguridad y protección de datos aplicables.")
        .set_style(style.clone());;
    
    sheet.add_merge_cells("A13:D16");
    sheet.get_cell_mut("A13").set_value("CARGUE EN SISTEMA Y DESTRUYA, NO COPIE, NO ENCRIPTE, NO APUNTE LA SEGURIDAD DEPENDE DE TODOS.")
        .set_style(style.clone());;


    book.new_sheet("Servidor#1").unwrap().get_cell_mut("A1").set_value("TEST1");

    // writer
    let path = std::path::Path::new("logcred_remd_ser_xxxxxxx.xlsx");
    let _ = writer::xlsx::write(&book, path);
}