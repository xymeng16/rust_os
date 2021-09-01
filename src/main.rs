#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unused_imports)]

// mod gdt;
// mod interrupts;
// mod serial;
// mod vga_buffer;

use bootloader::boot_info::{FrameBuffer, FrameBufferInfo};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use core::ptr::slice_from_raw_parts_mut;
use rust_os::println;
use x86_64::structures::paging::{Page, Translate};

entry_point!(kernel_main);

#[allow(unreachable_code)]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    use rust_os::memory;
    use x86_64::VirtAddr;
    use memory::BootInfoFrameAllocator;

    if let Some(fb) = boot_info.framebuffer.as_mut() {
        init(fb);
    }

    let mut physical_memory_offset: u64 = 0;
    if let Some(offset) = boot_info.physical_memory_offset.as_mut() {
        physical_memory_offset = *offset;
    }

    println!("Hello rust_os!");

    #[cfg(test)]
    test_main();

    rust_os::hlt_loop();
}

pub fn init(fb: &'static mut FrameBuffer) {
    let fb_info = fb.info();
    rust_os::vga_buffer::init_global_writer(fb.buffer_mut(), fb_info);

    rust_os::init();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    rust_os::hlt_loop();
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
