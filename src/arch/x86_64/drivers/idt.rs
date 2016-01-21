use core::mem;
use arch::x86_64::drivers::pic;

const IDTSIZE: usize = 256;

pub static mut IDT: [IDTEntry; IDTSIZE] = 
    [IDTEntry {base_lo: 0, base_hi: 0, select: 0, flags: 0,}; IDTSIZE];
pub static mut IDT_POINTER: IDTPointer = IDTPointer {base: 0, limit: 0};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IDTPointer {
    limit: u16,
    base: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IDTEntry {
    base_lo: u16,
    select: u16,
    flags: u8,
    base_hi: u16,
}

impl IDTEntry {
    pub fn set(&mut self, isr: unsafe extern fn(), select: u16, flags: u8) {
        let base = isr as u32;
        self.base_lo = (base & 0xffff) as u16;
        self.base_hi = ((base >> 16) & 0xffff) as u16;
        self.select = select;
        self.flags = flags;
    }
}

extern {
    fn isr_wrapper_kbd();
}

pub unsafe fn install_idt() {
    IDT_POINTER.limit = ((mem::size_of::<IDTEntry>() * IDTSIZE) - 1) as u16;
    IDT_POINTER.base = &IDT as *const [IDTEntry; IDTSIZE] as u32;

    IDT[33].set(isr_wrapper_kbd, 0x08, 0x8E);

    pic::PICS.lock().initialize();

    pic::PICS.lock().mask_all_irqs();
    pic::PICS.lock().unmask_irq(1); // unmask the keyboard irq

    asm!("lidt ($0)" :: "r" (IDT));
    asm!("sti");
}
