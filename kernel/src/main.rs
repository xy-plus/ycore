#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]

#[macro_use]
extern crate log;

#[macro_use]
mod console;

mod batch;
mod cpu;
mod lang_items;
mod sbi;
mod syscall;
mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

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
    if hartid == BOOT_HART_ID {
        clear_bss();
        console::init("info");
        info!("--- Hello, world! from cpu {} ---", hartid);
        trap::init();
        batch::init();
        cpu::broadcast_ipi(); // wake other core
        batch::run_next_app();
        // unreachable!
    }
    info!("--- Hello, world! from cpu {} ---", hartid);
    loop {}
}
