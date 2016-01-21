use arch::x86_64::drivers::pic;
use arch::x86_64::drivers::io_ports;

#[no_mangle]
pub unsafe extern fn _isr_kbd_handler() {
    let keyboard_port: io_ports::UnsafePort<u8> = io_ports::UnsafePort::new(0x60);
    let scancode = keyboard_port.read();
    println!("scancode {:?}", scancode);

    pic::PICS.lock().notify_end_of_interrupt(1);
}
