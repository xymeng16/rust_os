#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unused_imports)]

mod gdt;
mod interrupts;
mod serial;
mod vga_buffer;

use bootloader::boot_info::{FrameBuffer, FrameBufferInfo};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use core::ptr::slice_from_raw_parts_mut;

entry_point!(kernel_main);

#[allow(unreachable_code)]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(fb) = boot_info.framebuffer.as_mut() {
        init(fb.raw_buffer_info().0, fb.raw_buffer_info().1, fb.info());
    }

    println!("Hello rust_os!");
    x86_64::instructions::interrupts::int3();
    println!("provoking a deadlock");

    #[cfg(test)]
    test_main();

    rust_os::hlt_loop();
}

pub fn init(fb_start: u64, fb_len: usize, fb_info: FrameBufferInfo) {
    unsafe {
        vga_buffer::init_global_writer(
            &mut *slice_from_raw_parts_mut(fb_start as *mut u8, fb_len),
            fb_info,
        );
    }
    gdt::init();
    interrupts::init_idt();
    unsafe {
        interrupts::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
    // rust_os::init(fb_start, fb_len, fb_info);
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
