#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(stone_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use stone_os::{memory::BootInfoFrameAllocator, println};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use stone_os::memory;
    use x86_64::{structures::paging::Page, VirtAddr};
    println!("Hello World{}", "!");

    stone_os::init();

    #[cfg(test)]
    test_main();

    println!("Stones!");
    stone_os::hlt_loop()
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    stone_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    stone_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
