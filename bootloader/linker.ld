SECTIONS {
	. = 1M;

	.multiboot_header : {
		KEEP(*(.multiboot_header))
	}

	.text : ALIGN(4K) {
		*(.text .text.*)
	}
	.rodata : ALIGN(4K) {
		*(.rodata .rodata.*)
	}
	.data : ALIGN(4K) {
		*(.data .data.*)
	}
	.bss : ALIGN(4K) {
		*(.bss .bss.*)
	}
}
