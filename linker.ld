ENTRY(efi_main)
SECTIONS {
    .text : {
        *(.text*)
    }
    .data : {
        *(.data*)
    }
    .bss : {
        *(COMMON)
        *(.bss*)
    }
}
