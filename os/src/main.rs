#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::arch::global_asm;

#[path = "boards/qemu.rs"]
mod board;
#[macro_use]
mod console;
mod config;
mod lang_items;
mod lists;
mod loader;
mod mm;
mod sbi;
mod sync;
pub mod syscall;
pub mod task;
mod timer;
pub mod trap;

global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

#[allow(unused)]
fn clear_bss() {
    extern "C" {
        fn bss_start();
        fn bss_end();
    }

    let bss_start = bss_start as usize;
    let bss_end = bss_end as usize;

    (bss_start..bss_end).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

#[no_mangle]
fn main() -> ! {
    mm::init();
    lists::test();
    panic!("Unreachable in rust_main!");
}
