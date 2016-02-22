#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn, unique)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate multiboot2;
extern crate x86;
#[macro_use] extern crate bitflags;

#[macro_use]
mod arch;
mod memory;

use arch::x86_64::drivers::vga_terminal;
use arch::x86_64::drivers::pic;
use arch::x86_64::drivers::idt;
use arch::x86_64::drivers::keyboard;


#[no_mangle]
pub extern fn kernel_main(multiboot_header_address: usize) {
    vga_terminal::WRITER.lock().clear_screen();

    println!("lossy booting up! :D\n");

    let boot_info = unsafe { multiboot2::load(multiboot_header_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Multiboot2 header: Memory map tag required!");
    let elf_sections_tag = boot_info.elf_sections_tag().expect("Multiboot2 header: ELF sections tag required!");

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr)
        .max().unwrap();

    let multiboot_start = multiboot_header_address;
    let multiboot_end = multiboot_header_address + (boot_info.total_size as usize);

    println!("Memory areas (determined from Multiboot):");
    for area in memory_map_tag.memory_areas() {
        println!("    memory_area {{ start: 0x{:x}, length: 0x{:x} }}",
            area.base_addr, area.length);
    }

    println!("ELF sections (determined from Multiboot):");
    for section in elf_sections_tag.sections() {
        println!("    elf_section {{ addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x} }}", section.addr, section.size, section.flags)
    }

    println!("kernel_start = 0x{:x}, kernel_end = 0x{:x}", kernel_start, kernel_end);
    println!("multiboot_start = 0x{:x}, multiboot_end = 0x{:x}", multiboot_start, multiboot_end);

    let mut frame_allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, multiboot_start,
        multiboot_end, memory_map_tag.memory_areas());

    for i in 0.. {
        use memory::FrameAllocator;
        if let None = frame_allocator.allocate_frame() {
            println!("allocated {} frames", i);
            break;
        }
    }

    unsafe {
        idt::install_idt();
    }

    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    // set scary error colors
    vga_terminal::WRITER.lock().set_color_code(vga_terminal::ColorCode::new(vga_terminal::Color::White, vga_terminal::Color::Red));

    println!("\n\nPANIC in {} at line {}", file, line);
    println!("    {}", fmt);

    loop {} // spin forever
}
