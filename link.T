OUTPUT_FORMAT("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(_start)

PHDRS {
  text    PT_LOAD FLAGS(5);
  rodata  PT_LOAD FLAGS(4);
  data    PT_LOAD FLAGS(6);
}

SECTIONS {
  PROVIDE(__payload_start__ = 0x40010000);
  PROVIDE(__stack_top__     = 0x40010000);
  PROVIDE(__relocator__     = 0x4003F000);

  . = __payload_start__;

  /* App code */
  .text : {
    HIDDEN(__text_start__ = .);
    KEEP(*(.text.crt0))
    *(.text .text.*)
    HIDDEN(__text_end__ = .);
  } :text

  /* App rodata */
  .rodata : {
    *(.roda)
    *(.rodata .rodata.*)
    SORT(CONSTRUCTORS)
  } :rodata

  /* App data */
  .data : {
    *(.data .data*)
    CONSTRUCTORS
  } :data

  . = ALIGN(16);

  PROVIDE(__payload_end__ = ABSOLUTE(.));

  /* BSS section */
  .bss (NOLOAD) : {
    . = ALIGN(16);
    PROVIDE(__bss_start__ = ABSOLUTE(.));
    *(.bss .bss.*)
    *(COMMON)
    . = ALIGN(16);
    PROVIDE(__bss_end__ = ABSOLUTE(.));
  } :NONE

  /DISCARD/ : {
    *(.interp)
  }
}
