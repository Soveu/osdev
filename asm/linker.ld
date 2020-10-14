ENTRY(_start)
OUTPUT_FORMAT(elf32-i386)

SECTIONS {
    . = 1M;
    .boot : {
      boot.o (.boot)
    }

    .text   : ALIGN(4K) { *(.text) }
    .data   : ALIGN(4K) { *(.data) }
    .rodata : ALIGN(4K) { *(.rodata) }
    .bss    : ALIGN(4K) { *(.bss) }
}

