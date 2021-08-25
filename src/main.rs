#![no_std]
#![no_main]
#![feature(core_intrinsics)]

mod vga_buffer;

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use crate::vga_buffer::Writer;
use core::borrow::BorrowMut;
use core::fmt::Write;
use core::intrinsics::abort;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(fb) = boot_info.framebuffer.as_mut() {
        let info = fb.info().clone();
        // let mut writter = Writer::new(fb.buffer_mut(), info);
        // writter.write_str("Hello, world!");
        Writer::init_global_writer(fb.buffer_mut(), info);
        unsafe {write!((*(vga_buffer::WRITER.lock().as_ptr() as *mut Writer)), "TEST!");}
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // ! never return (diverging function)
    loop {}
}