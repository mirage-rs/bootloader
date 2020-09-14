OUTPUT_FORMAT("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(_start)

PHDRS {
  text    PT_LOAD FLAGS(5);
  rodata  PT_LOAD FLAGS(4);
  data    PT_LOAD FLAGS(6);
}

STACK_SIZE = 4K;

SECTIONS {
  /* The start address to the binary because we gain code execution through memory corruption. */
  PROVIDE(__start__     = 0x40010000);

  /* A location in upper IRAM where the payload relocator can be safely copied to. */
  PROVIDE(__relocator__ = 0x4003F000);

  . = __start__;

  /* The binary gets loaded to 0x40010000, but we should reserve 4K bytes for the stack. */
  PROVIDE(__stack_end__   = ABSOLUTE(.));
  PROVIDE(__stack_start__ = ABSOLUTE(__stack_end__ - STACK_SIZE));

  /* App code */
  .text :
  {
    HIDDEN(__text_start__ = .);
    KEEP(*(.text.rt))
    *(.text*)
    HIDDEN(__text_end__   = .);
  } :text

  /* App rodata */
  .rodata :
  {
    *(.roda)
    *(.rodata*)
    SORT(CONSTRUCTORS)
  } :rodata

  /* App data */
  .data :
  {
    *(.data*)
    CONSTRUCTORS
  } :data

  /* App init array */
  .init_array ALIGN(4) :
  {
    HIDDEN(__init_array_start__ = ABSOLUTE(.));
    KEEP(*(SORT_BY_INIT_PRIORITY(.init_array.*)))
    KEEP(*(.init_array))
    HIDDEN(__init_array_end__   = ABSOLUTE(.));
  } :data

  /* App fini array */
  .fini_array ALIGN(4) :
  {
    HIDDEN(__fini_array_start__ = ABSOLUTE(.));
    KEEP(*(SORT_BY_INIT_PRIORITY(.fini_array.*)))
    KEEP(*(.fini_array))
    HIDDEN(__fini_array_end__   = ABSOLUTE(.));
  } :data

  /* App BSS */
  .bss (NOLOAD) :
  {
    . = ALIGN(4);
    PROVIDE(__bss_start__ = ABSOLUTE(.));
    *(.bss*)
    *(COMMON)
    . = ALIGN(4);
    PROVIDE(__bss_end__   = ABSOLUTE(.));
  } :NONE

  . = ALIGN(16);
  PROVIDE(__end__ = ABSOLUTE(.));

  /DISCARD/ : {
    *(.interp)
  }
}
