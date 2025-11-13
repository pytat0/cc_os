#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cc_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use cc_os::println;
use core::panic::PanicInfo;

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    cc_os::init_idt();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cc_os::test_panic_handler(info)
}
