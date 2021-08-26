#![no_std]
#![no_main]
#![allow(unused_imports)]

use core::panic::PanicInfo;
use rust_os::{QemuExitCode, exit_qemu, serial_print, serial_println};
use bootloader::{BootInfo, entry_point_test};

entry_point_test!(ktest_main);

#[cfg(test)]
#[allow(unused_variables, unreachable_code)]
fn ktest_main(boot_info: &'static mut BootInfo) -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
#[allow(unreachable_code)]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    serial_print!("should_fail... ");
    assert_eq!(0, 1);
}