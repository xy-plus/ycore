use crate::cpu;
use crate::trap::TrapContext;
use core::cell::RefCell;
use lazy_static::*;
use spin::Mutex;

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;
const MAX_APP_NUM: usize = 16;
const MAX_CPU_NUM: usize = 4;
const APP_BASE_ADDRESS: usize = 0x80280000;
const APP_SIZE_LIMIT: usize = 0x20000;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACKs: [KernelStack; MAX_CPU_NUM] = [
    KernelStack {
        data: [0; KERNEL_STACK_SIZE],
    },
    KernelStack {
        data: [0; KERNEL_STACK_SIZE],
    },
    KernelStack {
        data: [0; KERNEL_STACK_SIZE],
    },
    KernelStack {
        data: [0; KERNEL_STACK_SIZE],
    },
];

static USER_STACKs: [UserStack; MAX_CPU_NUM] = [
    UserStack {
        data: [0; USER_STACK_SIZE],
    },
    UserStack {
        data: [0; USER_STACK_SIZE],
    },
    UserStack {
        data: [0; USER_STACK_SIZE],
    },
    UserStack {
        data: [0; USER_STACK_SIZE],
    },
];

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

struct AppManager {
    inner: RefCell<AppManagerInner>,
}
struct AppManagerInner {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManagerInner {
    pub fn print_app_info(&self) {
        info!("num_app = {}", self.num_app);
        for i in 0..self.num_app {
            info!(
                "app_{} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    fn app_base_addr() -> usize {
        return APP_BASE_ADDRESS + cpu::id() * APP_SIZE_LIMIT;
    }

    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            // panic!("All applications completed!");
            info!("No app to run!");
            loop {}
        }
        // clear icache
        llvm_asm!("fence.i" :::: "volatile");
        let app_base_addr_ = Self::app_base_addr();
        info!(
            "Loading app_{}: {:x}..{:x} to {:x}",
            app_id,
            self.app_start[app_id],
            self.app_start[app_id + 1], 
            app_base_addr_
        );
        // clear app area
        (app_base_addr_..(app_base_addr_ + APP_SIZE_LIMIT)).for_each(|addr| {
            (addr as *mut u8).write_volatile(0);
        });
        // core::ptr::write_bytes(app_base_addr_ as *mut u8, 0, APP_SIZE_LIMIT);
        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );
        let app_dst = core::slice::from_raw_parts_mut(app_base_addr_ as *mut u8, app_src.len());
        app_dst.copy_from_slice(app_src);
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
}

lazy_static! {
    static ref APP_MANAGER: Mutex<AppManager> = Mutex::new(AppManager {
        inner: RefCell::new({
            extern "C" {
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = unsafe { num_app_ptr.read_volatile() };
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =
                unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManagerInner {
                num_app,
                current_app: 0,
                app_start,
            }
        }),
    });
}

pub fn init() {
    print_app_info();
}

pub fn print_app_info() {
    APP_MANAGER.lock().inner.borrow().print_app_info();
}

pub fn run_next_app() -> ! {
    let app_manager = APP_MANAGER.lock();
    let current_app = app_manager.inner.borrow().get_current_app();
    unsafe {
        app_manager.inner.borrow().load_app(current_app);
    }
    app_manager.inner.borrow_mut().move_to_next_app();
    drop(app_manager);
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    let app_base_addr = AppManagerInner::app_base_addr();
    info!("switch to addr: {:x}", app_base_addr);
    unsafe {
        __restore(
            KERNEL_STACKs[cpu::id()].push_context(TrapContext::app_init_context(
                app_base_addr,
                USER_STACKs[cpu::id()].get_sp(),
            )) as *const _ as usize,
        );
    }
    panic!("Unreachable in batch::run_current_app!");
}
