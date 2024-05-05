//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM, syscall::{SYSCALL_EXIT, SYSCALL_GET_TIME, SYSCALL_TASK_INFO, SYSCALL_YIELD}, task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, TASK_MANAGER}, timer::get_time_us
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    TASK_MANAGER.increase_current_task_syscall_count(SYSCALL_EXIT);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    TASK_MANAGER.increase_current_task_syscall_count(SYSCALL_YIELD);
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }

    TASK_MANAGER.increase_current_task_syscall_count(SYSCALL_GET_TIME);
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// (the ch3 task seems fun hmmm) 
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");

    let _sys_ti = TASK_MANAGER.current_task_status();
    let _syscall_count_arr = TASK_MANAGER.current_task_syscall_count_array();

    unsafe {
        *_ti = TaskInfo {
            status: _sys_ti, // 测例调用时，当前任务一定是 Running 状态 
            syscall_times: _syscall_count_arr,
            time: 0,
        };
    }

    TASK_MANAGER.increase_current_task_syscall_count(SYSCALL_TASK_INFO);
    0
}
