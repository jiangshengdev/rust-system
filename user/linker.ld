ENTRY(_start)
BASE_ADDRESS = 0x80400000;

SECTIONS {
    . = BASE_ADDRESS;

    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    .bss : {
        bss_start = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        bss_end = .;
    }

    /DISCARD/ : {
        *(.eh_frame)
    }
}
