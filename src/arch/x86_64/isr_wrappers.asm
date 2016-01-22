global isr_wrapper_kbd

section .text
bits 64
align 8
isr_wrapper_kbd:
    extern _isr_kbd_handler
    call _isr_kbd_handler
    iretq
