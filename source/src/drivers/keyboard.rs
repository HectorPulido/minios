use core::arch::asm;

use crate::arch_x86_64::{
    idt::register_handler,
    pic::{inb, outb},
};
use crate::drivers::vga::{backspace_on_vga, write_byte, write_line, write_string, VGA_COLOR};

/// Mapa de scancodes -> ASCII (simplificado)
static SCANCODE_MAP: [Option<u8>; 128] = {
    let mut map = [None; 128];
    // Letras
    map[0x1E] = Some(b'a');
    map[0x30] = Some(b'b');
    map[0x2E] = Some(b'c');
    map[0x20] = Some(b'd');
    map[0x12] = Some(b'e');
    map[0x21] = Some(b'f');
    map[0x22] = Some(b'g');
    map[0x23] = Some(b'h');
    map[0x17] = Some(b'i');
    map[0x24] = Some(b'j');
    map[0x25] = Some(b'k');
    map[0x26] = Some(b'l');
    map[0x32] = Some(b'm');
    map[0x31] = Some(b'n');
    map[0x18] = Some(b'o');
    map[0x19] = Some(b'p');
    map[0x10] = Some(b'q');
    map[0x13] = Some(b'r');
    map[0x1F] = Some(b's');
    map[0x14] = Some(b't');
    map[0x16] = Some(b'u');
    map[0x2F] = Some(b'v');
    map[0x11] = Some(b'w');
    map[0x2D] = Some(b'x');
    map[0x15] = Some(b'y');
    map[0x2C] = Some(b'z');

    // Números
    map[0x02] = Some(b'1');
    map[0x03] = Some(b'2');
    map[0x04] = Some(b'3');
    map[0x05] = Some(b'4');
    map[0x06] = Some(b'5');
    map[0x07] = Some(b'6');
    map[0x08] = Some(b'7');
    map[0x09] = Some(b'8');
    map[0x0A] = Some(b'9');
    map[0x0B] = Some(b'0');

    // Espacio
    map[0x39] = Some(b' ');

    map
};

static mut INPUT_BUFFER: [u8; 128] = [0; 128];
static mut INPUT_LEN: usize = 0;

#[no_mangle]
extern "C" fn keyboard_interrupt_handler() {
    unsafe {
        let scancode = inb(0x60);

        // Si el bit más alto está en 1, es "key release"
        if scancode & 0x80 != 0 {
            // Ignoramos liberaciones en este ejemplo
            outb(0x20, 0x20); // EOI
            asm!("sti");
            return;
        }

        // Enter (scancode 0x1C)
        if scancode == 0x1C {
            write_byte(b'\n', VGA_COLOR);
            process_input();
            INPUT_LEN = 0;
        }
        // Backspace (scancode 0x0E)
        else if scancode == 0x0E {
            if INPUT_LEN > 0 {
                INPUT_LEN -= 1;
                backspace_on_vga();
            }
        }
        // Tecla normal
        else {
            if let Some(ch) = SCANCODE_MAP[scancode as usize] {
                if INPUT_LEN < INPUT_BUFFER.len() {
                    INPUT_BUFFER[INPUT_LEN] = ch;
                    INPUT_LEN += 1;
                    write_byte(ch, VGA_COLOR);
                }
            }
        }

        // EOI
        outb(0x20, 0x20);
        core::arch::asm!("sti");
    }
}

/// Procesa el contenido de INPUT_BUFFER cuando presionamos Enter
unsafe fn process_input() {
    // Convirtiendo a &str (asumiendo ASCII simple)
    let input_str = core::str::from_utf8_unchecked(&INPUT_BUFFER[..INPUT_LEN]);

    if input_str == "hola" {
        write_line("> Hola dev!", VGA_COLOR);
    } else if input_str == "clear" {
        crate::drivers::vga::clear_screen();
    } else if input_str.starts_with("echo") {
        let echo_str = &input_str[5..];
        write_line(echo_str, VGA_COLOR);
    } else {
        write_line("> Comando no reconocido", VGA_COLOR);
    }

    write_string("> ", VGA_COLOR);
}

/// Inicializa el teclado: registra el handler en la IDT
pub unsafe fn init_keyboard() {
    let handler_fn = keyboard_interrupt_handler as u64;
    // Vector 0x21 (33 decimal) es IRQ1 del teclado.
    register_handler(0x21, handler_fn as u64, 0x08, 0x8E00);
}
