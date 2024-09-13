extern crate alloc;
use core::fmt::Debug;

use alloc::boxed::Box;

use crate::utils::UNIXERROR;
#[derive(Debug)]
pub enum vnodetype
{
    DIRECTORY,
    FILE,
    SYMLINK
}
pub struct vfs {
    vfs_next: Box<vfs>,
    vfs_ops: Box<dyn Vfsops>,
    vnode: Box<vnode>,
}

#[derive(Debug)]
pub struct vnode {
    pub ops: Box<dyn Vnodeops>,
    pub flags: vnodetype,
    pub data: *mut u8
}
// i love boxes

pub trait Vnodeops {
    fn v_rdwr(&mut self, v: &mut vnode, sizeofbuf: usize, offset: usize, buf: &mut u8, rw: i32) -> Result<usize, UNIXERROR>;
    fn v_lookup(&mut self, v: &mut vnode, part: &str, l: &mut Option<&mut vnode>) -> UNIXERROR;
    fn v_filesz(&mut self, v: &mut vnode) -> Result<usize, UNIXERROR>;
}
impl Debug for dyn Vnodeops {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("no")
    }
}
trait Vfsops {}
static mut CUR_VFS: Option<&mut vfs> = None;
pub fn resolve_path(path: &str) -> Option<&mut vnode> {
    if path.is_empty() {
        return None;
    }
    let i = 0;

    if path.starts_with('/') && path.len() == 1 {
        unsafe {
            if let Some(q) = &mut CUR_VFS {
                return Some(&mut q.vnode);
            }
        }
    }
    return None;
}
