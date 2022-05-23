#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

#[macro_use]
mod console;
pub mod batch;
mod lang_items;
mod sbi;
mod sync;
pub mod syscall;
pub mod trap;

global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

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
    clear_bss();
    println!("[kernel] Hello, world!");
    trap::init();
    batch::init();
    batch::run_next_app();
}
