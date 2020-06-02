multiboot:
	nasm -f elf32 multiboot_header.asm
	nasm -f elf32 boot.asm
	ld -n -m elf_i386 --script=linker.ld multiboot_header.o boot.o -o kernel.bin
	rm boot.o multiboot_header.o
