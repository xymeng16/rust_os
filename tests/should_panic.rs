#![no_std]
#![no_main]
#![allow(unused_imports)]

#[cfg(test)]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[cfg(test)]
entry_point!(ktest_main);

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
