use core::arch::asm;

/// Escribe un byte (val) en un puerto (port)
pub unsafe fn outb(port: u16, val: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") val,
    );
}

/// Lee un byte desde un puerto (port)
pub unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    asm!(
        "in al, dx",
        in("dx") port,
        out("al") val,
    );
    val
}

/// Remapea el PIC maestro/esclavo para que las IRQ no colisionen con excepciones de la CPU.
pub unsafe fn remap_pic() {
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

/// Habilita la IRQ dada (por ejemplo, enable_irq(1) para teclado).
/// Si IRQ=1 => deshabilita el bit 1 (poniéndolo en 0) en la máscara del PIC maestro.
pub unsafe fn enable_irq(irq: u8) {
    // Leemos la máscara actual del PIC maestro
    let mut mask = inb(0x21);
    // Ponemos en 0 el bit correspondiente a la IRQ
    mask &= !(1 << irq);
    outb(0x21, mask);
}

pub unsafe fn enable_keyboard_irq() {
    outb(0x21, 0xFD);
}

/// Envía EOI al PIC (para indicar fin de la interrupción).
/// Útil si en algún momento lo quieres invocar manualmente.
pub unsafe fn send_eoi() {
    outb(0x20, 0x20); // EOI al maestro
                      // Si fuera necesario para IRQ>7, enviar EOI también al esclavo (0xA0)
}
