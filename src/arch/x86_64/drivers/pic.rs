// programmable interrupt controller driver

use arch::x86_64::drivers::io_ports::UnsafePort;
use spin::Mutex;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe {
    ChainedPics::new(0x20, 0x28) // 0x20 == 32, we're mapping up beyond the x86 fault codes.
});

const CMD_END_OF_INTERRUPT: u8 = 0x20;

const CONF_ICW1_INIT: u8 = 0x10;
const CONF_ICW1_ICW4: u8 = 0x01;
const CONF_ICW4_8086: u8 = 0x01;

pub struct Pic {
    offset: u8, // interrupt mapping offset
    command: UnsafePort<u8>,
    data: UnsafePort<u8>,
}

impl Pic {
    fn handles_interrupt(&self, interrupt: u8) -> bool {
        self.offset <= interrupt && interrupt < self.offset + 8
    }

    unsafe fn end_of_interrupt(&mut self) {
        self.command.write(CMD_END_OF_INTERRUPT);
    }

    pub unsafe fn set_mask(&mut self, mask: u8) {
        self.data.write(mask);
    }
}

pub struct ChainedPics {
    pub pics: [Pic; 2],
}

impl ChainedPics {
    pub const unsafe fn new(pic1_offset: u8, pic2_offset: u8) -> ChainedPics {
        ChainedPics {
            pics: [
                Pic {
                    offset: pic1_offset,
                    command: UnsafePort::new(0x20),
                    data: UnsafePort::new(0x21),
                },
                Pic {
                    offset: pic2_offset,
                    command: UnsafePort::new(0xA0),
                    data: UnsafePort::new(0xA1),
                },
            ],
        }
    }

    pub unsafe fn initialize(&mut self) {
        let mut io_wait_port: UnsafePort<u8> = UnsafePort::new(0x80);
        let mut io_wait = || { io_wait_port.write(0x00); };

        // save the interrupt masks - we'll restore them after initialization
        let mask1: u8 = self.pics[0].data.read();
        let mask2: u8 = self.pics[1].data.read();

        // tell the PICs that initialization data will follow on their data ports
        self.pics[0].command.write(CONF_ICW1_INIT + CONF_ICW1_ICW4);
        io_wait();
        self.pics[1].command.write(CONF_ICW1_INIT + CONF_ICW1_ICW4);
        io_wait();

        // send the PICs their offsets
        self.pics[0].data.write(self.pics[0].offset);
        io_wait();
        self.pics[1].data.write(self.pics[1].offset);
        io_wait();

        // send the PICs their chaining configuration
        self.pics[0].data.write(4);
        io_wait();
        self.pics[1].data.write(2);
        io_wait();

        // send the PICs their mode configuration (8086)
        self.pics[0].data.write(CONF_ICW4_8086);
        io_wait();
        self.pics[1].data.write(CONF_ICW4_8086);
        io_wait();

        // restore the interrupt masks
        self.pics[0].data.write(mask1);
        self.pics[1].data.write(mask2);
    }

    pub fn handles_interrupt(&self, interrupt: u8) -> bool {
        self.pics.iter().any(|pic| pic.handles_interrupt(interrupt))
    }

    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt: u8) {
        if self.handles_interrupt(interrupt) {
            if self.pics[1].handles_interrupt(interrupt) {
                self.pics[1].end_of_interrupt();
            }
            self.pics[0].end_of_interrupt();
        }
    }

    pub unsafe fn set_mask_for_irq(&mut self, irq: u8, masked: bool) {
        let pic: &mut Pic;
        let mapped_irq: u8;

        if irq < 8 {
            pic = &mut self.pics[0];
            mapped_irq = irq;
        } else {
            pic = &mut self.pics[1];
            mapped_irq = irq - 8;
        }

        let current_mask = pic.data.read();

        match masked {
            true => pic.data.write(current_mask | (1 << mapped_irq)),
            false => pic.data.write(current_mask & !(1 << mapped_irq)),
        }
    }

    pub unsafe fn mask_irq(&mut self, irq: u8) {
        self.set_mask_for_irq(irq, true);
    }

    pub unsafe fn unmask_irq(&mut self, irq: u8) {
        self.set_mask_for_irq(irq, false);
    }

    pub unsafe fn mask_all_irqs(&mut self) {
        for pic in &mut self.pics {
            pic.data.write(0xff);
        }
    }
}
