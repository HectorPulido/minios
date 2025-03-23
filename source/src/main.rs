#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_COLOR: u8 = 0x0F; // Color
static mut CURRENT_COL: usize = 0;
static mut CURRENT_ROW: usize = 0;

// -----------------------------------------
//  Definición de la IDT y sus entradas
// -----------------------------------------
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    options: u16,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    const fn missing() -> Self {
        IdtEntry {
            offset_low: 0,
            selector: 0,
            options: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }
    fn set_handler(&mut self, handler: u64, selector: u16, options: u16) {
        self.offset_low = (handler & 0xFFFF) as u16;
        self.offset_mid = ((handler >> 16) & 0xFFFF) as u16;
        self.offset_high = ((handler >> 32) & 0xFFFFFFFF) as u32;
        self.selector = selector;
        self.options = options;
        self.reserved = 0;
    }
}

// 256 entradas en la IDT
static mut IDT: [IdtEntry; 256] = [IdtEntry::missing(); 256];

// Descriptor
#[repr(C, packed)]
struct IdtDescriptor {
    limit: u16,
    base: u64,
}

// Carga la IDT usando lidt
unsafe fn load_idt() {
    let descriptor = IdtDescriptor {
        limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
        base: IDT.as_ptr() as u64,
    };
    core::arch::asm!(
        "lidt [{desc}]",
        desc = in(reg) &descriptor
    );
}

// -----------------------------------------
//  Acceso a puertos de E/S (para PIC y teclado)
// -----------------------------------------
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") val,
    );
}

unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    core::arch::asm!(
        "in al, dx",
        in("dx") port,
        out("al") val,
    );
    val
}

// -----------------------------------------
//  Remapeo del PIC (8259) para evitar colisiones
// -----------------------------------------
unsafe fn remap_pic() {
    // ICW1
    outb(0x20, 0x11);
    outb(0xA0, 0x11);
    // ICW2
    outb(0x21, 0x20);
    outb(0xA1, 0x28);
    // ICW3
    outb(0x21, 0x04);
    outb(0xA1, 0x02);
    // ICW4
    outb(0x21, 0x01);
    outb(0xA1, 0x01);

    // Máscaras: 0 => habilitado
    outb(0x21, 0x00);
    outb(0xA1, 0x00);
}

// -----------------------------------------
//  Habilitar sólo IRQ1 (teclado): 0xFD = 11111101
//  => bit 1 (IRQ1) en 0 => habilitado
// -----------------------------------------
unsafe fn enable_keyboard_irq() {
    outb(0x21, 0xFD);
}

// -----------------------------------------
//  Tabla scancodes -> ASCII (simplificada)
// -----------------------------------------
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

unsafe fn write_byte(byte: u8, color: u8) {
    if byte == b'\n' {
        CURRENT_COL = 0;
        CURRENT_ROW += 1;
        return;
    }

    let offset = (CURRENT_ROW * 80 + CURRENT_COL) as usize;
    *VGA_BUFFER.offset(offset as isize * 2) = byte;
    *VGA_BUFFER.offset(offset as isize * 2 + 1) = color;

    CURRENT_COL += 1;
}

unsafe fn write_string(s: &str, color: u8) {
    for byte in s.bytes() {
        match byte {
            // printable ASCII byte or newline
            0x20..=0x7e | b'\n' => write_byte(byte, color),
            // not part of printable ASCII range
            _ => write_byte(0xfe, color),
        }
    }
}

unsafe fn write_line(s: &str, color: u8) {
    write_string(s, color);
    CURRENT_COL = 0;
    CURRENT_ROW += 1;
}

unsafe fn clear_screen() {
    for i in 0..80 * 25 {
        *VGA_BUFFER.offset(i as isize * 2) = b' ';
        *VGA_BUFFER.offset(i as isize * 2 + 1) = VGA_COLOR;
    }
    CURRENT_COL = 0;
    CURRENT_ROW = 0;
}

unsafe fn wait() {
    for _ in 0..10_000_000 {
        core::hint::spin_loop();
    }
}

unsafe fn process_input() {
    let input_str = core::str::from_utf8_unchecked(&INPUT_BUFFER[..INPUT_LEN]);
    if input_str == "hola" {
        write_line("> Hola dev!", VGA_COLOR);
    } else if input_str == "clear" {
        clear_screen();
    } else if input_str.starts_with("echo") {
        let echo_str = &input_str[5..];
        write_line(echo_str, VGA_COLOR);
    } else {
        write_line("> Comando no reconocido", VGA_COLOR);
    }

    write_string("> ", VGA_COLOR);
}

static mut INPUT_BUFFER: [u8; 128] = [0; 128];
static mut INPUT_LEN: usize = 0;

#[no_mangle]
extern "C" fn keyboard_interrupt_handler() {
    unsafe {
        let scancode = inb(0x60);

        if scancode & 0x80 != 0 {
            outb(0x20, 0x20);
            core::arch::asm!("sti");
            return;
        }

        if scancode == 0x1C {
            write_byte(b'\n', VGA_COLOR);
            unsafe {
                process_input();
            }
            INPUT_LEN = 0;
        } else if scancode == 0x0E {
            // Backspace
            if INPUT_LEN > 0 {
                CURRENT_COL -= 1;
                write_byte(b' ', VGA_COLOR);
                CURRENT_COL -= 1;
                INPUT_LEN -= 1;
            }
        } else {
            if let Some(ch) = SCANCODE_MAP[scancode as usize] {
                if INPUT_LEN < INPUT_BUFFER.len() {
                    INPUT_BUFFER[INPUT_LEN] = ch;
                    INPUT_LEN += 1;
                    write_byte(ch, VGA_COLOR);
                }
            }
        }

        outb(0x20, 0x20);
        core::arch::asm!("sti");
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        remap_pic();
        for i in 0..256 {
            IDT[i] = IdtEntry::missing();
        }
        let handler_fn = keyboard_interrupt_handler as u64;
        IDT[0x21].set_handler(handler_fn as u64, 0x08, 0x8E00);

        load_idt();
        enable_keyboard_irq();
        core::arch::asm!("sti");
    }

    unsafe {
        write_line("Sistema minimal Rust OS", VGA_COLOR);
        write_line("Escribe algo y presiona Enter...", VGA_COLOR);
        write_string("> ", VGA_COLOR);
    }

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
