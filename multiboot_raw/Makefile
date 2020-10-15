run:
	yasm -f elf64 asm.asm
	ld --script=linker.ld asm.o
	qemu-system-x86_64 -kernel a.out
