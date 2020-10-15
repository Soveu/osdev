OUTPUT_FORMAT(binary)

SECTIONS {
    . = 2M;
    BIN_START = .;

    .bootloader : { *(.bootloader) }

    .text   : ALIGN(4K) { 
      TEXT_START = .;
      *(.text .text.*) 
    }

    .rodata : ALIGN(4K) {
      RODATA_START = .;
      *(.rodata .rodata.*)
    }

    .got    : ALIGN(4K) {
      GOT_START = .;
      *(.got .got.*)
    }

    .data   : ALIGN(4K) {
      DATA_START = .;
      *(.data .data.*)
      BIN_END = .;
    }

    .bss    : ALIGN(4K) { 
      BSS_START = .;
      *(.bss .bss.*)

      . = ALIGN(4K);
      *(.stack)
    }

    IMAGE_END = .;
}

