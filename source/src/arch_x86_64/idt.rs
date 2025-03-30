use core::arch::asm;

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
        Self {
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

#[repr(C, packed)]
struct IdtDescriptor {
    limit: u16,
    base: u64,
}

// Carga la IDT usando lidt
pub unsafe fn load_idt() {
    let descriptor = IdtDescriptor {
        limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
        base: IDT.as_ptr() as u64,
    };
    asm!(
        "lidt [{desc}]",
        desc = in(reg) &descriptor
    );
}

/// Función pública para inicializar la IDT entera (dejándola vacía inicialmente).
pub unsafe fn init_idt() {
    // Dejar en missing() todas las entradas
    for i in 0..256 {
        IDT[i] = IdtEntry::missing();
    }

    // Cargar la IDT en el registro correspondiente
    load_idt();
}

/// Registra una rutina de interrupción en el vector dado (por ejemplo, 0x21 para el teclado).
pub unsafe fn register_handler(vector: u8, handler_fn: u64, selector: u16, options: u16) {
    IDT[vector as usize].set_handler(handler_fn, selector, options);
}
