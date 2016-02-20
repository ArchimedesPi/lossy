use core::mem;
use core::intrinsics::transmute;
use arch::x86_64::drivers::pic;

const IDTSIZE: usize = 256;

pub static mut IDT: [IDTEntry; IDTSIZE] = 
    [IDTEntry {offset_lo: 0, offset_mid: 0, offset_hi:0,
     select: 0, flags: 0,
     zero: 0, zero_: 0}; IDTSIZE];
pub static mut IDT_POINTER: IDTPointer = IDTPointer {offset: 0, limit: 0};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct IDTPointer {
    limit: u16,
    offset: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct IDTEntry {
    offset_lo: u16,
    select: u16,
    zero: u8,
    flags: u8,
    offset_mid: u16,
    offset_hi: u32,
    zero_: u8,
}

impl IDTEntry {
    pub fn set(&mut self, isr: unsafe extern fn(), select: u16, flags: u8) {
        let offset = isr as u64;
        println!("isr offset: {:x}", offset);

        self.offset_lo = (offset & 0xffff) as u16;
        println!("isr offset low: {:x}", self.offset_lo);
        self.offset_mid = ((offset >> 16) & 0xffff) as u16;
        println!("isr offset mid: {:x}", self.offset_mid);
        self.offset_hi = (offset >> 32) as u32;
        println!("isr offset high: {:x}", self.offset_hi);
        self.select = select;
        self.flags = flags;
    }
}

unsafe fn lidt(idt: &IDTPointer) {
    println!("lidt {:?}", *idt);
    asm!("lidt ($0)" :: "r"(idt));
}

unsafe fn enable() {
    asm!("sti");
}

unsafe fn disable() {
    asm!("cli");
}

unsafe extern fn key_hit() {
    asm!("iretq");
}

pub unsafe fn install_idt() {
    IDT_POINTER = IDTPointer {
        limit: ((mem::size_of::<IDTEntry>() * IDTSIZE) - 1) as u16,
        offset: transmute::<&[IDTEntry; IDTSIZE], u64>(&IDT),
    };

    IDT[33].set(key_hit, 0x08, 0x8E);

    pic::PICS.lock().initialize();

    pic::PICS.lock().mask_all_irqs();
    pic::PICS.lock().unmask_irq(1);

    lidt(&IDT_POINTER);

    enable();
}
