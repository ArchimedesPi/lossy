/* linker script - build ELF executable from ELF object files */

ENTRY(start) /* the entry point of the executable - the bootloader will jump here */

SECTIONS {
    . = 1M; /* load the kernel at 1M */

    .boot : /* boot header section */
    {
        /* add multiboot header */
        KEEP(*(.multiboot_header)) /* KEEP to make sure the gc doesn't delete it */
    }

    .text : /* code section */
    {
        *(.text .text.*) /* pull in all input .text sections */
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data.rel.ro : {
        *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
    }
}
