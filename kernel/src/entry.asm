    .section .text.entry
    .globl _start
_start:
    # 每个核启动栈栈顶地址计算公式：栈区域开头地址+(核ID+1)*启动栈大小
    add t0, a0, 1
    slli t0, t0, 16
    la sp, boot_stack
    add sp, sp, t0
    call rust_main

    .section .bss.stack
    .align 12   # page align
    .globl boot_stack
boot_stack:
    .space 4096 * 16 * 4
    .globl boot_stack_top
boot_stack_top:
