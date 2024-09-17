#![feature(rustc_private)]
use crate::{fs::{
    vfs::{vfs, vnode, CUR_VFS},
    PosixFile,
}, mem::{phys::{HDDM_OFFSET, PMM}, virt::{cur_pagemap, PageMap}}, utils::{rdmsr, wrmsr}};
extern crate alloc;
use crate::idt::Registers_Exception;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use owo_colors::OwoColorize;
use spin::Mutex;
use crate::serial_println;
use alloc::vec::Vec;

use core::cell::RefCell;
use hashbrown::HashMap;
enum HandleType {
    File(Box<PosixFile>),
}
#[repr(C, packed)]
struct fpu_state {
    fcw: u16,
    rev0: u32,
    rev: u16,
    rev2: u64,
    rev3: u64,
    mxcsr: u32
}
pub struct cpu_context {
    frame: *mut Registers_Exception,
    // fpu_state: fpu_state
}
impl cpu_context {
    fn new(entry: u64, user_mode: bool, rsp: u64) -> Self {
        if user_mode {
            let mut construct = Registers_Exception
                {
                    int: 0,
                    r10: 0,
                    r11: 0,
                    r12: 0,
                    r13: 0,
                    r14: 0,
                    r15: 0,
                    r8: 0,
                    r9: 0,
                    rax: 0,
                    rbp: 0,
                    rbx: 0,
                    rcx: 0,
                    rdi: 0,
                    rdx: 0,
                    rip: entry as usize,
                    rsi: 0,
                    error_code: 0,
                    cs: 0x40 | (3),
                    ss: 0x38 | (3),
                    rflags: 0x202,
                    rsp: rsp as usize
                };
            Self {
                frame: &mut construct as *mut Registers_Exception,
                
            }
        }
        else {
            let mut construct = Registers_Exception
                {
                    int: 0,
                    r10: 0,
                    r11: 0,
                    r12: 0,
                    r13: 0,
                    r14: 0,
                    r15: 0,
                    r8: 0,
                    r9: 0,
                    rax: 0,
                    rbp: 0,
                    rbx: 0,
                    rcx: 0,
                    rdi: 0,
                    rdx: 0,
                    rip: entry as usize,
                    rsi: 0,
                    error_code: 0,
                    cs: 0x28,
                    ss: 0x30,
                    rflags: 0x202,
                    rsp: rsp as usize
                };
                Self {
                    frame: &mut construct as *mut Registers_Exception,
                    
                }
        }
        
    }
}
impl Thread {
    fn new(process: Arc<Mutex<process>>, context: cpu_context, name: String) -> Self{
        Self {
            context: context,
            name: name,
            tid: 0,
            gs_base: 0 as *mut perthreadcpuinfo,
            fs: 0,
            process: process
        }
    }
}
impl process {
    fn new(vfs: Option<*mut vfs>, pagemap: Option<*mut PageMap>, name: String) -> Self {
        Self {
            cwd: unsafe {if let Some(vfs) = vfs {(*vfs).vnode.clone()} else{None}},
            vfs: None,
            pagemap: None,
            pid: 0,
            handles: HashMap::new(),
            name: name,
            threads: Vec::new()

        }
    }
}
impl perthreadcpuinfo {
    fn new(kernel_stack_ptr: *mut u8, user_stack_ptr: *mut u8, on_thread: Arc<Mutex<Thread>>) -> Box<Self>{
        Box::new(Self {
            kernel_stack_ptr: kernel_stack_ptr,
            user_stack_ptr: user_stack_ptr,
            current_thread: on_thread
        })
    }
}
use alloc::sync::Arc;
#[repr(C, packed(8))]
pub struct perthreadcpuinfo {
    kernel_stack_ptr: *mut u8,
    user_stack_ptr: *mut u8,
    current_thread: Arc<Mutex<Thread>>
}

pub struct process {
    vfs: Option<*mut vfs>,
    pagemap: Option<*mut PageMap>,
    cwd: Option<Arc<Mutex<dyn vnode>>>,
    pid: u64,
    handles: HashMap<usize, HandleType>,
    name: String,
    threads: Vec<Arc<Mutex<Thread>>>,
}


struct Thread {
    name: String,
    tid: u64,
    gs_base: *mut perthreadcpuinfo,
    fs: u64,
    context: cpu_context,
    process: Arc<Mutex<process>>
}

struct cpu_queue {
    queue: Option<Vec<Arc<Mutex<process>>>>,
    lapic_id: u32
}
pub struct CpuSQueue {
    queue: Vec<Arc<Mutex<cpu_queue>>>
}
impl CpuSQueue {
    pub fn create_queue(&mut self, lapic_id: u32) {
        let queu = cpu_queue { queue: Some(Vec::new()), lapic_id: lapic_id };
        serial_println!("created queue with lapic id {}", queu.lapic_id);
        self.queue.push(Arc::new(Mutex::new(queu)));
        
    }
}
pub static mut ITIS: Option<CpuSQueue> = None;
use alloc::string::ToString;
extern "C" fn lol() -> !
{
    println!("hi");
    loop {

    }
}
pub fn sched_init() {
    
    unsafe {
        ITIS = Some(
            CpuSQueue { queue: Vec::new() }
        );
    }
    // create queue for boot cpu
    
    unsafe {
        ITIS.as_mut().unwrap().create_queue(0);
    }
    let mut process = Arc::new(Mutex::new(process::new(unsafe {Some(CUR_VFS.as_mut().unwrap())}, unsafe {Some(cur_pagemap.as_mut().unwrap())}, "hi".to_string())));
    let kstack = unsafe {(PMM.alloc().unwrap() as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u8};
    let mut context = cpu_context::new(lol as u64, false, unsafe {kstack.offset(4096) as u64});
    let mut thre = Arc::new(Mutex::new(Thread::new(process.clone(), context, "kthread".to_string())));
    let per = perthreadcpuinfo::new(unsafe {kstack.offset(4096)}, 0 as *mut u8, thre.clone());
    thre.lock().gs_base = Box::into_raw(per);
    wrmsr(0xC0000101, thre.lock().gs_base as u64);
    let y = rdmsr(0xC0000101);
    println!("wrote GS base: {:#x}", y);
    unsafe {
        let mut o =&mut  ITIS.as_mut()
            .unwrap().queue;
        
            let mut j = o.first();
            if let Some(jj) = j.as_mut() {
                
                jj.lock().queue.as_mut().unwrap().push(process.clone());
            }
        process.lock().threads.insert(0, thre);
    }
}
pub fn real_sched_init() {

}
#[derive(Debug)]
struct fake_cpuqueue {
    lapic_id: u32
}
#[derive(Debug)]
struct fakequeues {
    queue: Vec<fake_cpuqueue>
}
#[test]
fn sched_queuetest() {
    
    extern crate libc;
    use std::io;
    use std::ptr;
    use libc::{mmap, munmap, sysconf, _SC_PAGESIZE, PROT_READ, PROT_WRITE, MAP_PRIVATE, MAP_ANONYMOUS, MAP_FAILED};
    use std::println;
    use std::thread;
    let mut fake = cpu_queue {queue: Some(Vec::new()), lapic_id: 0};
    
    let mut process = Arc::new(Mutex::new(process::new(None, unsafe {None}, "hi".to_string())));
     // Map one page of memory into the process (anonymous memory, not backed by a file)
     let addr = unsafe {mmap(
        ptr::null_mut(),        // Let the OS choose the address
        4096,              // Size of the mapping (one page)
        PROT_READ | PROT_WRITE, // Read/write access
        MAP_PRIVATE | MAP_ANONYMOUS, // Private anonymous mapping (not backed by a file)
        -1,                     // No file descriptor (since this is anonymous memory)
        0,                      // Offset (ignored for anonymous mapping)
    )};

    // Check if mmap failed
    if addr == MAP_FAILED {
        assert_eq!(1, 2);
    }

    // Cast the returned address to a mutable u8 pointer
    let data = addr as *mut u8;
    
    let kstack = data;
    let mut context = cpu_context::new(lol as u64, false, unsafe {kstack.offset(4096) as u64});
    let mut thre = Arc::new(Mutex::new(Thread::new(process.clone(), context, "kthread".to_string())));
    let per = perthreadcpuinfo::new(unsafe {kstack.offset(4096)}, 0 as *mut u8, thre.clone());
    thre.lock().gs_base = Box::into_raw(per);
    
    serial_println!("GS base is {y}");
    unsafe {
        let o = fake.queue.unwrap();
        
            let mut j = o;
            process.lock().threads.insert(0, thre);
            j.push(process);
        
    }
    unsafe {munmap(
        addr as *mut _,
        4096
    )};
    
}
use crate::println;
pub fn save_context(frame: *mut Registers_Exception, context: &mut cpu_context)
{
    
    unsafe {
        context.frame = frame;
    }
    // do xrrstore
}
pub fn switch_context(frame: *mut Registers_Exception, context: &mut cpu_context) -> *mut Registers_Exception{
    
    unsafe {
        context.frame
    }
    
}
#[no_mangle]
pub fn scheduletask(regs: *mut Registers_Exception) -> Option<*mut Registers_Exception>{
    // pick a queue
    let que = unsafe {
        &mut ITIS.as_mut().unwrap().queue
    };
    let mut got_you = que.pop();
    if got_you.is_none() {
        return None;
    }
    let mut r = got_you;
    if r.is_none() {
        return None;
    }
    let mut new_process = r.as_mut().unwrap().lock().queue.as_mut().unwrap().pop();
    if new_process.is_none() {
        return None;
    }
    let mut new_thread = new_process.as_mut().unwrap().lock().threads.pop();
    if new_thread.is_none() {
        return None;
    }
    let info = unsafe {
        let e = rdmsr(0xC0000101) as *mut perthreadcpuinfo;
        e
        
    };
    // get current thread
    let thr = unsafe {
        
        (*info).current_thread.clone()
    };
    save_context(regs, &mut thr.lock().context);
    let fs = rdmsr(0xC0000100);
    thr.lock().fs = fs;
    let cc = switch_context(regs, &mut new_thread.as_mut().unwrap().lock().context);
    wrmsr(0xC0000100, new_thread.as_ref().unwrap().lock().fs);
    wrmsr(0xC0000102, new_thread.as_ref().unwrap().lock().gs_base as u64);
    
    let o = thr;
    new_process.as_mut().unwrap().lock().threads.insert(0, o.clone());
    r.as_mut().unwrap().lock().queue.as_mut().unwrap().insert(0, o.lock().process.clone());
    Some(cc)
    
}