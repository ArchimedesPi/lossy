use core::marker::PhantomData;

pub trait CpuInOut {
    unsafe fn port_in(port: u16) -> Self;
    unsafe fn port_out(port: u16, data: Self);
}

impl CpuInOut for u8 {
    unsafe fn port_in(port: u16) -> u8 {
        let result: u8;
        asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
        result
    }

    unsafe fn port_out(port: u16, data: u8) {
        asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(data) :: "volatile");
    }
}

impl CpuInOut for u16 {
    unsafe fn port_in(port: u16) -> u16 {
        let result: u16;
        asm!("inw %dx, %ax" : "={ax}"(result) : "{dx}"(port) :: "volatile");
        result
    }

    unsafe fn port_out(port: u16, data: u16) {
        asm!("outw %ax, %dx" :: "{dx}"(port), "{ax}"(data) :: "volatile");
    }
}

impl CpuInOut for u32 {
    unsafe fn port_in(port: u16) -> u32 {
        let result: u32;
        asm!("inl %dx, %eax" : "={eax}"(result) : "{dx}"(port) :: "volatile");
        result
    }
    unsafe fn port_out(port: u16, data: u32) {
        asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(data) :: "volatile");
    }
}

pub struct Port<T: CpuInOut> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: CpuInOut> Port<T> {
    pub const unsafe fn new(port: u16) -> Port<T> {
        Port { port: port, phantom: PhantomData }
    }

    pub fn read(&self) -> T {
        unsafe {
            T::port_in(self.port)
        }
    }

    pub fn write(&mut self, data: T) {
        unsafe {
            T::port_out(self.port, data);
        }
    }
}

pub struct UnsafePort<T: CpuInOut> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: CpuInOut> UnsafePort<T> {
    pub const unsafe fn new(port: u16) -> UnsafePort<T> {
        UnsafePort { port: port, phantom: PhantomData }
    }

    pub fn read(&self) -> T {
        unsafe {
            T::port_in(self.port)
        }
    }

    pub fn write(&mut self, data: T) {
        unsafe {
            T::port_out(self.port, data);
        }
    }
}

