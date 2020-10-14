bits 32

global _start
extern rust_start 

section .boot

dd 0x1BADB002     ; magic number (multiboot)
dd 0              ; no flags
dd -0x1BADB002    ; checksum

_start:
  hlt
  jmp _start

