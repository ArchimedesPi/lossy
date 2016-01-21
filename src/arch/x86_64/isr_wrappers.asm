global isr_wrapper_kbd

section .text
bits 64
isr_wrapper_kbd:
    extern _isr_kbd_handler
    call _isr_kbd_handler
    iretq
