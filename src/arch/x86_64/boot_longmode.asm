global long_mode_start

section .text
bits 64 ; yay we're in 64-bit mode now, that was a lot of work :)
long_mode_start:
    ; print `OKAY` to screen (using an extended register)
    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax ; quadword move
    hlt
