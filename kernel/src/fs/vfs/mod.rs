extern crate alloc;
use core::any::Any;
use core::fmt::{Debug, Pointer};
use core::ops::Index;

use alloc::boxed::Box;
use alloc::string::ToString;
use crate::println;
pub struct vfs {
    vfs_next: Box<vfs>,
    vfs_ops: Box<dyn vfsops>,
    vnode: Box<vnode>
}

#[derive(Debug)]
pub struct vnode {
    ops: Box<dyn vnodeops>,
}
// i love boxes

pub trait vnodeops {
    fn v_rdwr(&mut self, v: &mut vnode, sizeofbuf:usize, offset: usize, buf: &mut u64, rw: i32);
    fn v_lookup(&mut self, v: &mut vnode, part: &str, l: &mut Option<&mut vnode>);
}
impl Debug for dyn vnodeops
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("no")
    }
}
trait vfsops {

}
static mut CUR_VFS: Option<&mut vfs> = None;
pub fn resolve_path(path: &str) -> Option<&mut vnode>
{
    if path.is_empty() {
        return None
    }
    let mut i = 0;
    
    if path.starts_with('/') && path.len() == 1
    {
        unsafe {
            if let Some(q) = &mut CUR_VFS
            {
                return Some(&mut q.vnode);
            }
        }
    }
    return None;
    

    
}