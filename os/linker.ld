ENTRY(start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    kernel_start = .;

    .text : {
        text_start = .;
        *(.text.entry)
        *(.text .text.*)
        . = ALIGN(4K);
        text_end = .;
    }

    .rodata : {
        rodata_start = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4K);
        rodata_end = .;
    }

    .data : {
        data_start = .;
        *(.data.stack)
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4K);
        data_end = .;
    }

    .bss : {
        bss_start = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        . = ALIGN(4K);
        bss_end = .;
    }

    kernel_end = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
