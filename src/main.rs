#![no_std]
#![no_main]  
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod vga_buffer;

// Rebuild with: cargo bootimage
// Run with:
// qemu-system-x86_64 -display gtk -drive format=raw,file=target/x86_64-hagoll_os/debug/bootimage-hagoll_os.bin

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello again, some numbers: {} {}", 42, 1.337);
    println!("This works.");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }
}