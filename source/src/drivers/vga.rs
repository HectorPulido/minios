// Dirección base de memoria de texto VGA
pub const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
// Color por defecto (blanco sobre negro, por ejemplo)
pub const VGA_COLOR: u8 = 0x0F;

static mut CURRENT_COL: usize = 0;
static mut CURRENT_ROW: usize = 0;

/// Limpia la pantalla
pub unsafe fn clear_screen() {
    for i in 0..(80 * 25) {
        *VGA_BUFFER.offset(i as isize * 2) = b' ';
        *VGA_BUFFER.offset(i as isize * 2 + 1) = VGA_COLOR;
    }
    CURRENT_COL = 0;
    CURRENT_ROW = 0;
}

/// Escribe un solo byte en la posición actual (con color por defecto).
pub unsafe fn write_byte(byte: u8, color: u8) {
    if byte == b'\n' {
        CURRENT_COL = 0;
        CURRENT_ROW += 1;
        return;
    }

    let offset = (CURRENT_ROW * 80 + CURRENT_COL) as isize;
    *VGA_BUFFER.offset(offset * 2) = byte;
    *VGA_BUFFER.offset(offset * 2 + 1) = color;

    CURRENT_COL += 1;
    if CURRENT_COL >= 80 {
        CURRENT_COL = 0;
        CURRENT_ROW += 1;
    }
}

/// Escribe una cadena
pub unsafe fn write_string(s: &str, color: u8) {
    for &byte in s.as_bytes() {
        match byte {
            // ASCII printable o newline
            0x20..=0x7e | b'\n' => write_byte(byte, color),
            // Otra cosa -> mostramos 0xfe
            _ => write_byte(0xfe, color),
        }
    }
}

/// Escribe una cadena y luego un salto de línea
pub unsafe fn write_line(s: &str, color: u8) {
    write_string(s, color);
    write_byte(b'\n', color);
}

/// Mueve el cursor hacia atrás una posición y borra el carácter
pub unsafe fn backspace_on_vga() {
    if CURRENT_COL > 0 {
        CURRENT_COL -= 1;
    } else {
        if CURRENT_ROW > 0 {
            CURRENT_ROW -= 1;
            CURRENT_COL = 79; // final de la línea anterior
        }
    }
    let offset = (CURRENT_ROW * 80 + CURRENT_COL) as isize;
    *VGA_BUFFER.offset(offset * 2) = b' ';
    *VGA_BUFFER.offset(offset * 2 + 1) = VGA_COLOR;
}
