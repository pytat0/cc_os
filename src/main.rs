#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cc_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use cc_os::println;
use x86_64::registers::control::{Cr0, Cr3};
use core::panic::PanicInfo;

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    cc_os::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());


/*
    let ptr = 0xdeadbeaf as *mut u8;
    #unsafe { *ptr = 42; }


    let ptr = 0x2031b2 as *mut u8;
    unsafe {let x = *ptr;}
    println!("read worked");

    unsafe {*ptr = 42;}
    println!("write worked");
*/

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    cc_os::hlt_loop();

}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    cc_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cc_os::test_panic_handler(info)
}
