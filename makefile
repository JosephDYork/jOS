IMG = jOS.img

all: $(IMG)

boot.bin: bootld1.asm
	nasm -f bin bootld1.asm -o boot.bin

$(IMG): boot.bin
	# Creates an empty 1.44MB floppy image
	dd if=/dev/zero of=$(IMG) bs=512 count=2880
	# Write boot sector
	dd if=boot.bin of=$(IMG) conv=notrunc

clean:
	rm -f boot.bin $(IMG)
