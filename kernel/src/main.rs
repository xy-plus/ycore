#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod cpu;
mod lang_items;
mod sbi;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

const BOOT_HART_ID: usize = 0;

#[no_mangle]
pub fn rust_main(hartid: usize, _device_tree_paddr: usize) -> ! {
    unsafe {
        cpu::set_cpu_id(hartid);
    }
    if hartid != BOOT_HART_ID {
        println!("Hello, world! from cpu {}", hartid);
        cpu::send_ipi(hartid + 1);
        loop {}
    }
    clear_bss();
    println!("Hello, world! from cpu {}", hartid);
    cpu::send_ipi(hartid + 1);
    loop {}
    panic!("Shutdown machine!");
}
