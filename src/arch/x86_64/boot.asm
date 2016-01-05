; entry point for x86_64 machines
; sets up basic stuff to get the machine to boot, switches into
; real (and then long) mode, sets up for Rust code,
; and calls into the kernel.

global start ; set up a global label for grub to jump to

section .text
bits 32 ; we're still in protected mode, so we're using 32-bit instructions
start:
    mov dword [0xb8000], 0x2f4b2f4f ; this prints OK (prepacked VGA buffer value)

    hlt ; halt the CPU
