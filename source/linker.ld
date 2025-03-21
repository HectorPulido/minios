ENTRY(_start)
SECTIONS {
  /* Coloca la sección .multiboot al inicio y usa KEEP para conservarla */
  .multiboot ALIGN(4) : {
    KEEP(*(.multiboot))
  }
  . = ALIGN(4);

  .text : { *(.text*) }
  .rodata : { *(.rodata*) }
  .data : { *(.data*) }
  .bss : { *(.bss*) }
}