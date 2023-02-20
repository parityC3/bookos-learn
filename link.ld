/* LM3S6965マイクロコントローラのメモリレイアウト */
/* 1K = 1 KiBi = 1024バイト */
MEMORY
{
  FLASH : ORIGIN = 0x00000000 + 16K, LENGTH = 512K - 16K
  RAM : ORIGIN = 0x20000000, LENGTH = 192K
}

/* エントリポイントはリセットハンドラです */
ENTRY(Reset);

EXTERN(RESET_VECTOR);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    /* 1つ目のエントリ。スタックポインタの初期値 */
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    /* 2つ目のエントリ。リセットベクタ */
    KEEP(*(.vector_table.reset_vector));

    KEEP(*(.vector_table.exceptions));
  } > FLASH

  .text :
  {
    *(.text .text.*);
  } > FLASH

  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

  .bss (NOLOAD):
  {
    _sbss = .;
    *(.bss .bss.*);
    _ebss = .;
  } > RAM

  .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
  {
    _sdata = .;
    *(.data .data.*);
    _edata = .;
  } > RAM

  _sidata = LOADADDR(.data);

  .app_stack ALIGN(0x08):
  {
    *(.app_stack .app_stack.*);
  } > RAM

  _heap_start = ADDR(.app_stack) + SIZEOF(.app_stack);

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}

PROVIDE(NMI = DefaultExceptionHandler);
PROVIDE(HardFault = DefaultExceptionHandler);
PROVIDE(MemManage = DefaultExceptionHandler);
PROVIDE(BusFault = DefaultExceptionHandler);
PROVIDE(UsageFault = DefaultExceptionHandler);
PROVIDE(SVCall = DefaultExceptionHandler);
PROVIDE(PendSV = DefaultExceptionHandler);
PROVIDE(SysTick = DefaultExceptionHandler);