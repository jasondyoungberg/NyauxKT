extern crate alloc;
use core::ffi::CStr;
use core::fmt::Debug;
use core::ops::Deref;

use alloc::boxed::Box;

use crate::fs::tmpfs::tmpfsdir;
use crate::println;
use crate::utils::UNIXERROR;
use alloc::rc::Rc;
use core::cell::RefCell;

pub trait vnode {
    fn lookup(&self, child: &str) -> Result<Rc<RefCell<dyn vnode>>, UNIXERROR>;
    fn read(&self, buf: &mut [u8], offset: usize, count: usize) -> Result<usize, UNIXERROR>;
    fn write(&mut self, buf: &[u8], offset: usize, count: usize) -> Result<usize, UNIXERROR>;
    fn mkdir(&mut self, name: &str) -> Result<Rc<RefCell<dyn vnode>>, UNIXERROR>;
    fn create(&mut self, name: &str) -> Result<Rc<RefCell<dyn vnode>>, UNIXERROR>;
    fn get_attrib(&self) -> Result<&vnodeattributetable, UNIXERROR>;
}

// i love boxes

pub trait vfsops {}
pub struct vfs {
    pub ops: Option<Box<dyn vfsops>>,
    pub vnode: Option<Rc<RefCell<dyn vnode>>>,
    pub next: Option<Box<vfs>>,
}
pub struct vnodeattributetable {
    pub size: usize,
    pub TYPE: VNODEFLAGS,
}

use alloc::string::String;
use alloc::string::ToString;

use alloc::vec::Vec;

use super::tmpfs;
use super::VNODEFLAGS;
pub static mut CUR_VFS: Option<vfs> = None;
impl Default for vfs {
    fn default() -> Self {
        vfs {
            ops: None,
            vnode: None,
            next: None,
        }
    }
}
pub fn get_list(path: &str) -> Vec<&str> {
    let mut q: Vec<&str> = path.split('/').collect();
    q.retain(|x| *x != "");
    return q;
}
pub fn resolve_path_absolute(
    path: &str,
    get_parent: bool,
) -> Result<Rc<RefCell<dyn vnode>>, UNIXERROR> {
    let mut cur_dir = unsafe { &mut CUR_VFS.as_mut().unwrap().vnode };
    let w = get_list(path);
    for (idx, i) in w.iter().enumerate() {
        let res = cur_dir.as_mut().unwrap().borrow_mut().lookup(i);
        if get_parent && w.get(idx + 1).is_none() {
            break;
        }
        if let Ok(ress) = res {
            *cur_dir = Some(ress);
        }
    }
    if let Some(qq) = cur_dir {
        return Ok(qq.clone());
    }
    return Err(UNIXERROR::ENOENT);
}
#[test]
fn path() {
    assert_eq!(["test", "hey", "yo"], *get_list("/test/hey/yo/"));

    assert_eq!(["usr", "bin", "bash"], *get_list("/usr/bin/bash"));

    let mut new = tmpfsdir::default();
    if let Ok(q) = new.create("hi") {
        assert_eq!(1, 1);
        let mut buf = "hello world!\n".as_bytes();
        let o = q.borrow_mut().write(&mut buf, 0, buf.len()).unwrap();
        assert_eq!(1, 1);
        let mut buf = [0; 256];
        let qowowwo = buf.len();
        let h = q.borrow_mut().read(&mut buf, 0, qowowwo).unwrap();
        let ok = CStr::from_bytes_until_nul(&buf).unwrap().to_str().unwrap();
        assert_eq!("hello world!\n", ok);

        let tt = new.lookup("hi").unwrap();
        let mut ttq = [0; 256];
        let wo = ttq.len();
        tt.borrow_mut().read(&mut ttq, 1, wo).unwrap();
        let better = CStr::from_bytes_until_nul(&ttq).unwrap().to_str().unwrap();
        assert_eq!("ello world!\n", better);
    }
    unsafe {
        CUR_VFS = Some(vfs {
            ops: None,
            vnode: None,
            next: None,
        })
    }
    unsafe { CUR_VFS.as_mut().unwrap().vnode = Some(Rc::new(RefCell::new(new))) };
    if let Ok(w) = resolve_path_absolute("/hi", false) {
        let mut ttq = [0; 256];
        let len = ttq.len();
        w.borrow_mut().read(&mut ttq, 0, len).unwrap();
        let better = CStr::from_bytes_until_nul(&ttq).unwrap().to_str().unwrap();
        assert_eq!("hello world!\n", better);
    }
}
// impl vnode {
//     fn vfs_create(&mut self, path: &str, typ: vnodetype, res: &mut Option<&mut vnode>) -> UNIXERROR {
//         if path.is_empty() {
//             return UNIXERROR::EINVAL;
//         }
//         let mut q = resolve_path(path);
//         if let Some(q) = q{

//         }
//         else {
//             return UNIXERROR::ENOENT;
//         }
//         return UNIXERROR::ESUCCESS;
//     }
// }
