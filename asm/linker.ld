ENTRY(bootstrap_start)
OUTPUT_FORMAT(elf32-i386)

SECTIONS {
    . = 1M;
    .boot : {
      boot.o (.text)
    }
    .text   BLOCK(4K) : ALIGN(4K) { 
      *(EXCLUDE_FILE(*boot.o) .text)
      *(.text) 
    }

    .data   BLOCK(4K) : ALIGN(4K) { *(.data) }
    .rodata BLOCK(4K) : ALIGN(4K) { *(.rodata) }
    .bss    BLOCK(4K) : ALIGN(4K) { *(.bss) }
}

