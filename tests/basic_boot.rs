#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point_test, BootInfo};
use core::panic::PanicInfo;
use rust_os::{println, serial_print};

entry_point_test!(ktest_main);
#[cfg(test)]
#[allow(unused_variables, unreachable_code)]
fn ktest_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(fb) = boot_info.framebuffer.as_mut() {
        rust_os::init(fb.raw_buffer_info().0, fb.raw_buffer_info().1, fb.info());
    }
    test_main();
    loop {}
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
