; **********************************************************
; Bootld1.asm :: A simple bootloader for the jOS kernel
; 
; Instructional Pet Operating system
; **********************************************************

org		0x7c00		; This code tells our assembler (NASM) that all addresses are relative to 0x7C00
bits		16		; Set us to good old 16 bit real mode

start:
	cli			; Clear all interrups
	xor ax, ax
	mov ds, ax
	mov es, ax
	mov ss, ax
	mov sp, 0x7C00
	sti
	
	mov si, msg
.print:
	lodsb
	or al, al
	jz halt
	mov ah, 0x0E
	int 0x10
	jmp .print

halt:
	cli
	hlt


msg db "Hello from floppy!", 0


; Pad boot sector to 512 bytes
times 510 - ($-$$) db 0		; pad us out so we are 512 bytes
dw 0xAA55			; boot sig
