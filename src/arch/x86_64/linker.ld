/* linker script - build ELF executable from ELF object files */

ENTRY(start) /* the entry point of the executable - the bootloader will jump here */

SECTIONS {
    . = 1M; /* load the kernel at 1M */

    .boot : /* boot header section */
    {
        /* add multiboot header */
        *(.multiboot_header)
    }

    .text : /* code section */
    {
        *(.text) /* pull in all input .text sections */
    }
}
