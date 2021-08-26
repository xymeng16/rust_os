#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod serial;
mod vga_buffer;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use vga_buffer::{init_global_writer, print_global_writer_info};

entry_point!(kernel_main);

#[allow(unreachable_code)]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {

    if let Some(fb) = boot_info.framebuffer.as_mut() {
        let info = fb.info().clone();
        init_global_writer(fb.buffer_mut(), info);
    }
    rust_os::init(/*boot_info*/);
    print_global_writer_info();
    println!("Hello rust_os!");
    // x86_64::instructions::interrupts::int3();
    // unsafe {
    //     *(0x0 as *mut u64) = 0;
    // }
    #[cfg(test)]
    test_main();

    // println!("It did not crash!");
    loop {}
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