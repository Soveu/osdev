section .multiboot_header

dd 0x1BADB002     ; magic number (multiboot 2)
dd 0              ; no flags
dd -0x1BADB002    ; checksum

