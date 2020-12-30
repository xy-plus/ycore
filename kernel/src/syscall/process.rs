use crate::batch::run_next_app;

pub fn sys_exit(xstate: i32) -> ! {
    info!("Application exited with code {}", xstate);
    run_next_app()
}
