#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

#[macro_use]
mod console;
mod lang_items;
mod sbi;

global_asm!(include_str!("entry.S"));

fn clear_bss() {
    extern "C" {
        fn bss_start();
        fn bss_end();
    }

    let bss_start = bss_start as usize;
    let bss_end = bss_end as usize;

    (bss_start..bss_end).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
fn main() -> ! {
    extern "C" {
        fn text_start();
        fn text_end();

        fn rodata_start();
        fn rodata_end();

        fn data_start();
        fn data_end();

        fn boot_stack();
        fn boot_stack_top();

        fn bss_start();
        fn bss_end();
    }

    clear_bss();

    println!("Hello, world!");

    let text_start = text_start as usize;
    let text_end = text_end as usize;

    let rodata_start = rodata_start as usize;
    let rodata_end = rodata_end as usize;

    let data_start = data_start as usize;
    let data_end = data_end as usize;

    let boot_stack = boot_stack as usize;
    let boot_stack_top = boot_stack_top as usize;

    let bss_start = bss_start as usize;
    let bss_end = bss_end as usize;

    println!(".text [{:#x}, {:#x})", text_start, text_end);
    println!(".rodata [{:#x}, {:#x})", rodata_start, rodata_end);
    println!(".data [{:#x}, {:#x})", data_start, data_end);
    println!("boot_stack [{:#x}, {:#x})", boot_stack, boot_stack_top);
    println!(".bss [{:#x}, {:#x})", bss_start, bss_end);

    panic!("Shutdown machine!");
}
