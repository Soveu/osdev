bits 32

extern IMAGE_END
extern BIN_END

section .bootloader
multiheader:

dd 0x1BADB002               ; magic
dd (1 << 16)                ; flags
dd -0x1BADB002 - (1 << 16)  ; checksum

dd multiheader              ; header_addr
dd multiheader              ; load_addr
dd BIN_END                  ; load_end_addr
dd IMAGE_END                ; bss_end_addr
dd _start                   ; entry

section .text
_start:
  mov eax, 0x2F412F41
  mov [0xB8000], eax
  hlt
  jmp _start

section .data
dd BIN_END                  ; load_end_addr
dd IMAGE_END                ; bss_end_addr
dd 0x41414141
dd 0x41414141

section .bss
align 4096
p4_table:
  resb 4096
p3_table:
  resb 4096
p2_table:
  resb 4096

