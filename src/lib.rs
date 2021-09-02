#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unused_imports)]

extern crate alloc;

pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga_buffer;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

use core::panic::PanicInfo;

use bootloader::boot_info::{FrameBuffer, FrameBufferInfo};
use core::mem;
use core::ptr::slice_from_raw_parts_mut;
use vga_buffer::{init_global_writer, Writer, WRITER};

#[cfg(test)]
entry_point!(ktest_main);

#[cfg(test)]
#[allow(unused_variables)]
fn ktest_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(fb) = boot_info.framebuffer.as_mut() {
        let fb_info = fb.info();

        unsafe {
            vga_buffer::init_global_writer(fb.buffer_mut(), fb_info);
        }
    }
    init();
    test_main();
    hlt_loop();
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe {
        interrupts::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
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

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
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

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
