ASM=yasm
KERNELPATHDEBUG=target/amd64-kernel-target/debug/libkernel.a
KERNELPATHRELEASE=target/amd64-kernel-target/release/libkernel.a

release:
	cargo build --release
	$(ASM) -f elf64 asm/boot.asm
	ld -n --script=asm/linker.ld boot.o $(KERNELPATHRELEASE) -o kernel.bin
	rm boot.o

debug:
	cargo build
	$(ASM) -f elf64 asm/boot.asm
	ld -n --script=asm/linker.ld boot.o $(KERNELPATHDEBUG) -o kernel.bin
	rm boot.o

clear:
	rm boot.o kernel.bin

run:
	qemu-system-x86_64 -kernel kernel.bin -m 1G -enable-kvm -cpu host

