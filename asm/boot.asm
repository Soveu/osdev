bits 32

extern rust_start

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
  ; https://intermezzos.github.io/book/first-edition/paging.html
  call setup_paging

  ; set up GDT, we only really need this for long mode
  lgdt [gdt64.pointer]
  mov ax, gdt64.data
  mov ss, ax
  mov ds, ax
  mov es, ax
  jmp gdt64.code:pog_mode_start

setup_paging:
  mov eax, p3_table
  or eax, 3
  mov [p4_table + 0], eax

  mov eax, p2_table
  or eax, 3
  mov [p3_table + 0], eax

  mov ecx, 0
map_p2_table:
  mov eax, 0x200000
  mul ecx
  or eax, 131 ;0b10000011
  mov [p2_table + ecx * 8], eax

  inc ecx
  cmp ecx, 512
  jne map_p2_table

  mov eax, p4_table
  mov cr3, eax
  ; enable PAE
  mov eax, cr4
  or eax, 1 << 5
  mov cr4, eax
  ; set long mode bit
  mov ecx, 0xC0000080
  rdmsr
  or eax, 1 << 8
  wrmsr
  ; enable paging
  mov eax, cr0
  or eax, 1 << 31
  or eax, 1 << 16
  mov cr0, eax

  ret





section .bss
align 4096

p4_table:
  resb 4096
p3_table:
  resb 4096
p2_table:
  resb 4096

section .rodata
gdt64:
  dq 0
.code: equ $ - gdt64
  dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53)
.data: equ $ - gdt64
  dq (1<<44) | (1<<47) | (1<<41)
.pointer:
  dw .pointer - gdt64 - 1
  dq gdt64




; Long mode stuff
section .text
bits 64

pog_mode_start:
  mov rax, 0x2F412F412F412F41 ;AAAA
  mov [0xB8000], rax

  mov rax, 0x200000 * 510
  xor rsp, rsp
  mov rsp, rax

  call rust_start

  hlt
  jmp pog_mode_start

