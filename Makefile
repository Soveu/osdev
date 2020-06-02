ASM=yasm
multiboot:
	$(ASM) -f elf64 multiboot_header.asm
	$(ASM) -f elf64 boot.asm
	ld -n --script=linker.ld multiboot_header.o boot.o -o kernel.bin

clear:
	rm boot.o multiboot_header.o

run:
	qemu-system-x86_64 -kernel kernel.bin

