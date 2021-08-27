#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod gdt;
mod interrupts;
mod serial;
mod vga_buffer;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

#[allow(unreachable_code)]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init(boot_info);
    println!("Hello rust_os!");
    x86_64::instructions::interrupts::int3();
    #[cfg(test)]
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
    unsafe { interrupts::PICS.lock().initialize(); }
    // x86_64::instructions::interrupts::enable();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
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
