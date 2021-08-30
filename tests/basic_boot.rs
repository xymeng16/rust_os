#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::boot_info::FrameBuffer;
#[cfg(test)]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::{println, serial_print};

#[cfg(test)]
entry_point!(ktest_main);

#[cfg(test)]
#[allow(unused_variables, unreachable_code)]
fn ktest_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(fb) = boot_info.framebuffer.as_mut() {
        init(fb);
    }
    test_main();
    loop {}
}

pub fn init(fb: &'static mut FrameBuffer) {
    let fb_info = fb.info();

    rust_os::vga_buffer::init_global_writer(fb.buffer_mut(), fb_info);

    rust_os::init();
}

#[allow(unused_variables, dead_code)]
fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    serial_print!("test_println... ");
    println!("test_println output");
}
