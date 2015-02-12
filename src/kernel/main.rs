#![nostd]
#![allow(ctypes)]

#![no_mangle]
#![no_split_stack]
pub fn main() {
	/* Do nothing here yet.
	   This will make the `call main` in boot.asm exit and the execution will continue,
	   so the processor will end up idling (see boot.asm).
	   */
}