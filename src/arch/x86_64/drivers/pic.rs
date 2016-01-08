// programmable interrupt controller driver
// be very, very, very, very careful with this
// literally everything is marked unsafe for a /very/ good reason.

use arch::x86_64::drivers::io_ports::UnsafePort;
use spin::Mutex;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe {
    ChainedPics::new(0x20, 0x28)
});

const CMD_END_OF_INTERRUPT: u8 = 0x20;

const CONF_ICW1_INIT: u8 = 0x10;
const CONF_ICW1_ICW4: u8 = 0x01;
const CONF_ICW4_8086: u8 = 0x01;

struct Pic {
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
}

pub struct ChainedPics {
    pics: [Pic; 2],
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
}
