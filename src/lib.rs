#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unused_imports)]

pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

use bootloader::{entry_point_test, BootInfo};
use core::panic::PanicInfo;

use bootloader::boot_info::FrameBufferInfo;
use core::mem;
use vga_buffer::{init_global_writer, Writer, WRITER};

entry_point_test!(ktest_main);
#[cfg(test)]
#[allow(unused_variables)]
fn ktest_main(boot_info: &'static mut BootInfo) -> ! {
    init(boot_info);
    test_main();
    loop {}
}

pub fn init(boot_info: &'static mut BootInfo) {
    if let Some(fb) = boot_info.framebuffer.as_mut() {
        let info = fb.info().clone();
        vga_buffer::init_global_writer(fb.buffer_mut(), info);
    }
    interrupts::init_idt();
    gdt::init();
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[allow(unreachable_code)]
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }

    loop {}
}
