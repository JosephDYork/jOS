IMG = jOS.img

all: bin/$(IMG)

bin/boot.bin: asm/bootld1.asm
	nasm -f elf32 asm/bootld1.asm -o bin/boot.o
	ld -melf_i386 -Ttext 0x7c00 bin/boot.o -o bin/boot.elf
	objcopy -O binary bin/boot.elf bin/boot.bin

bin/$(IMG): bin/boot.bin
	dd if=/dev/zero of=bin/$(IMG) bs=512 count=2880 # Creates an empty 1.44MB floppy image
	dd if=bin/boot.bin of=bin/$(IMG) conv=notrunc # Write boot sector

clean:
	rm -f bin/*
