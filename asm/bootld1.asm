; **********************************************************
; Bootld1.asm :: A simple bootloader for the jOS kernel
;
; Instructional Operating system
; **********************************************************

[bits 16]
global _start

_start:
    jmp 0x0000:_main

_main:                         ; 1. Normalize segments and set stack
    cli                         ; Disable interrupts during setup
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, [_start - 1]        ; Stack grows down from 0x7C00
    sti                         ; Re-enable interrupts

    cld                         ; Clear direction flag (forward copy)
    mov si, 0x7C00              ; Source: BIOS load address
    mov di, 0x0600              ; Destination: Conventional safe memory
    mov cx, 256                 ; 256 words = 512 bytes
    rep movsw                   ; Perform the copy

    ; Calculate the offset of 'relocated_code' and add it to 0x0600
    jmp 0x0000:(relocated_code - _start + 0x0600) ; 3. Jump to the new location

relocated_code:
    mov ah, 0x17                ; Set screen output type
    int 0x10

    mov si, status_string
    call clearscr
    call printstr
    call get_a20_state
    call halt

; =============== Global Descriptor Table Start ===================

gdt_start:
gdt_null:
    dd 0x0
    dd 0x0          ; 8-byte

gdt_code:
    dw 0xffff       ; limit
    dw 0x0          ; base 
    db 0x0          ; base
    db 10011010b    ; P, DPL, S, Type flags
    db 11001111b    ; G, D/B, L, AVL flags, limit 
    db 0x0          ; base 

gdt_data:
    dw 0xffff
    dw 0x0 
    db 0x0 
    db 10010010b    ; Type flag is diff
    db 11001111b 
    db 0x0 

gdt_end:
gdt_descriptor:
    ; size (16-bit) + addr (32-bit)
    dw gdt_end - gdt_start - 1
    dd gdt_start

; some useful information
CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start

; =============== Global Descriptor Table End ===================
; ============== Global Utility Functions Start ==================

halt:
    cli
    hlt

clearscr:
.begin:
    pusha

.scroll:
    mov dx, 184Fh   ; (79, 24) Set lower-right corner (DH=row, DL=column)
    xor cx, cx      ; (0, 0) Set upper-left corner (CH=row, CL=column)
    mov bh, 07h     ; Set background attribute (e.g., white on black)
    mov ax, 0700h   ; AH=07h (Scroll window down), AL=00h (clear entire window)
    int 10h         ; Call the BIOS video interrupt

.reset_cursor:
    mov ah, 02h     ; AH=02h (Set cursor position)
    xor bh, bh      ; BH=00h (Display page number)
    xor dx, dx      ; DL=00h (Column), DH=00h (Row)
    int 10h         ; Call the BIOS video interrupt

.done:
    popa
    ret

printstr: ; Print the string stored in si
.begin:
    pusha
    mov ah, 0x0E

.loop:
    lodsb
    or al, al
    jz .done
    int 0x10
    jmp .loop

.done:
    mov al, 10
    int 0x10
    mov al, 13
    int 0x10

    popa
    ret

get_a20_state:
    pushf
    push si
    push di
    push ds
    push es
    cli

    mov ax, 0x0000                  ;   0x0000:0x0500(0x00000500) -> ds:si
    mov ds, ax
    mov si, 0x0500

    not ax                      ;   0xffff:0x0510(0x00100500) -> es:di
    mov es, ax
    mov di, 0x0510

    mov al, [ds:si]                 ;   save old values
    mov byte [.BufferBelowMB], al
    mov al, [es:di]
    mov byte [.BufferOverMB], al

    mov ah, 1
    mov byte [ds:si], 0
    mov byte [es:di], 1
    mov al, [ds:si]
    cmp al, [es:di]                 ;   check byte at address 0x0500 != byte at address 0x100500
    jne .exit
    dec ah

.exit:
    mov al, [.BufferBelowMB]
    mov [ds:si], al
    mov al, [.BufferOverMB]
    mov [es:di], al
    shr ax, 8                   ;   move result from ah to al register and clear ah

    or al, al
    jz .done
    jmp .printa20

.printa20:
    mov si, a20_status_string
    call printstr

.done:
    pop es
    pop ds
    pop di
    pop si
    popf
    ret

    .BufferBelowMB: db 0
    .BufferOverMB   db 0

; ============== Global Utility Functions End ==================

a20_status_string db "[ Stage0: OK ] Bootloader A20 line: Enabled", 0
status_string db "[ Stage0: OK ] Bootloader relocated to 0x0600", 0

; --- Padding ---
times 510 - ($-$$) db 0         ; Pad to 510 bytes
dw 0xAA55                       ; Boot signature
