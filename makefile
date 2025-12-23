IMG = jOS.img

all: bin/$(IMG)

bin/boot.bin: asm/bootld1.asm
	nasm -f bin asm/bootld1.asm -o bin/boot.bin

bin/$(IMG): bin/boot.bin
	# Creates an empty 1.44MB floppy image
	dd if=/dev/zero of=bin/$(IMG) bs=512 count=2880
	# Write boot sector
	dd if=bin/boot.bin of=bin/$(IMG) conv=notrunc

clean:
	rm -f bin/boot.bin bin/$(IMG)
