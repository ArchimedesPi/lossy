global long_mode_start


section .text
bits 64 ; yay we're in 64-bit mode now, that was a lot of work :)
long_mode_start:
    extern kernel_main
    call kernel_main

.os_returned:
    mov rax, 0x4f724f204f534f4f
    mov [0xb8000], rax
    mov rax, 0x4f724f754f744f65
    mov [0xb8008], rax
    mov rax, 0x4f214f644f654f6e
    mov [0xb8010], rax
    hlt
