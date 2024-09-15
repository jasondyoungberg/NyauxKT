use crate::fs::{
    vfs::{vfs, vnode},
    PosixFile,
};
extern crate alloc;
use crate::idt::Registers_Exception;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;

use core::cell::RefCell;
use hashbrown::HashMap;
enum HandleType {
    File(Box<PosixFile>),
}
struct cpu_context {
    frame: *mut Registers_Exception,
    // fpu_state: todo!
}
#[repr(C, packed)]
struct perthreadcpuinfo {
    kernel_stack_ptr: *mut u8,
    user_stack_ptr: *mut u8,
}
struct process {
    vfs: *mut vfs,
    pagemap: u64,
    cwd: Rc<RefCell<dyn vnode>>,
    pid: u64,
    handles: HashMap<usize, HandleType>,
    name: String,
    threads: Vec<Thread>,
}
struct Thread {
    name: String,
    tid: u64,
    gs_base: *mut perthreadcpuinfo,
    context: *mut cpu_context,
    next: Option<Box<Thread>>
}

struct cpu_queue {
    queue: Option<Box<Thread>>
}
struct CpuSQueue {
    queue: Vec<cpu_queue>
}
