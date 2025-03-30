#![no_std]
#![no_main]

mod arch_x86_64;
mod drivers;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Importamos desde nuestros módulos
use arch_x86_64::{
    idt::init_idt,
    pic::{enable_irq, enable_keyboard_irq, remap_pic},
    pic::{inb, outb},
};
use drivers::{
    keyboard::init_keyboard,
    vga::{clear_screen, write_line, write_string},
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // 1) Remapeamos el PIC
        remap_pic();

        // 2) Inicializamos la IDT
        init_idt();

        // 3) Inicializamos el teclado (registra su ISR)
        init_keyboard();

        // 4) Habilitamos IRQ1 (teclado)
        enable_keyboard_irq();

        // 5) Activamos interrupciones
        core::arch::asm!("sti");
    }

    // 6) Probamos la salida por VGA
    unsafe {
        // clear_screen();

        const VGA_COLOR: u8 = 0x0F; // Color blanco
        write_line("Sistema minimal Rust OS", VGA_COLOR);
        write_line("Escribe algo y presiona Enter...", VGA_COLOR);
        write_string("> ", VGA_COLOR);
    }

    // Bucle infinito
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
