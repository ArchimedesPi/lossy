#![feature(lang_items)]
#![feature(const_fn, unique)]
#![no_std]

extern crate rlibc;

mod drivers;

#[no_mangle]
pub extern fn kernel_main() {
    drivers::vga_terminal::test();

    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() {}
