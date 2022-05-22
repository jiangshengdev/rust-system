use core::arch::asm;

use lazy_static::*;

use crate::sync::UPSafeCell;
use crate::trap::TrapContext;

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;
const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let kernel_stack_top = self.get_sp();
        let context_size = core::mem::size_of::<TrapContext>();

        let cx_ptr = (kernel_stack_top - context_size) as *mut TrapContext;

        unsafe {
            *cx_ptr = cx;
        }

        unsafe {
            let context = cx_ptr.as_mut().unwrap();
            context
        }
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn print_app_info(&self) {
        info!("[kernel] num_app = {}", self.num_app);

        for i in 0..self.num_app {
            trace!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            panic!("All applications completed!");
        }

        info!("[kernel] Loading app_{}", app_id);
        asm!("fence.i");

        let app_base_address = APP_BASE_ADDRESS as *mut u8;

        core::slice::from_raw_parts_mut(app_base_address, APP_SIZE_LIMIT).fill(0);

        let app_start = self.app_start[app_id];
        let app_end = self.app_start[app_id + 1];

        let app_start_address = app_start as *const u8;
        let app_len = app_end - app_start;

        let app_src = core::slice::from_raw_parts(app_start_address, app_len);
        let app_dst = core::slice::from_raw_parts_mut(app_base_address, app_src.len());

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
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
                fn _num_app();
            }

            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();

            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =
                core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);

            app_start[..=num_app].copy_from_slice(app_start_raw);

            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

pub fn init() {
    print_app_info();
}

pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}

pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();

    unsafe {
        app_manager.load_app(current_app);
    }

    app_manager.move_to_next_app();
    drop(app_manager);

    extern "C" {
        fn __restore(cx_addr: usize);
    }

    let user_stack_top = USER_STACK.get_sp();
    let context = TrapContext::app_init_context(APP_BASE_ADDRESS, user_stack_top);
    let context_addr = KERNEL_STACK.push_context(context) as *const _ as usize;

    unsafe {
        __restore(context_addr);
    }

    panic!("Unreachable in batch::run_current_app!");
}
