// tp is unused in kernel space
pub unsafe fn set_cpu_id(cpu_id: usize) {
    llvm_asm!("mv tp, $0" : : "r"(cpu_id));
}

pub fn id() -> usize {
    let cpu_id;
    unsafe {
        llvm_asm!("mv $0, tp" : "=r"(cpu_id));
    }
    cpu_id
}

pub fn send_ipi(cpu_id: usize) {
    super::sbi::send_ipi(1 << cpu_id);
}

pub fn broadcast_ipi() {
    super::sbi::send_ipi(usize::max_value());
}
