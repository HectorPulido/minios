#![no_std]
#![no_main]

mod vga_buffer;
use vga_buffer::{Buffer, Color, ColorCode, ScreenChar, Writer, BUFFER_HEIGHT, BUFFER_WIDTH};

#[no_mangle]
#[used]
#[link_section = ".multiboot"]
pub static MULTIBOOT_HEADER: [u32; 3] = [
    0x1BADB002,                   // Magic number requerido
    0x0,                          // Flags
    0x1BADB002u32.wrapping_neg(), // Checksum: -(magic + flags)
];

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const VGA_BUFFER: u64 = 0xb8000;

fn delay() {
    for _ in 0..100_000_000 {
        unsafe { core::arch::asm!("nop") }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Referencia explícita para forzar la inclusión
    let _ = &MULTIBOOT_HEADER;

    let color_code = ColorCode::new(Color::White, Color::Black);
    let buffer: &'static mut Buffer = unsafe { &mut *(VGA_BUFFER as *mut Buffer) };
    let mut writer = Writer::new(color_code, buffer);

    // Limpia todo el buffer (pinta toda la pantalla de espacios)
    writer.clear_screen();

    writer.write_string("Hello w");

    loop {}
}
