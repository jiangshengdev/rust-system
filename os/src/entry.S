    .section .text.entry
    .global _start
_start:
    la sp, boot_stack_top
    call main

    .section .data.stack
    .global boot_stack
boot_stack:
    .space 4096 * 16
    .global boot_stack_top
boot_stack_top:
