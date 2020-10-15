bits 32

global _start
extern rust_start 

section .boot

dd 0x1BADB002     ; magic number (multiboot)
dd 0              ; no flags
dd -0x1BADB002    ; checksum

_start:
  mov eax, 0x2F692F48
  mov [0xb8000], eax
  hlt
  jmp _start


bits 64
section .text

longmode_start:
  hlt
  jmp longmode_start
