extern crate alloc;
use alloc::boxed::Box;
struct vfs {
    vfs_next: Box<vfs>,
    vfs_ops: Box<dyn vfsops>,
    vnode: Box<vnode>
}
struct vnode {
    ops: Box<dyn vnodeops>,
}
// i love boxes
trait vnodeops {
    fn v_rdwr(&mut self, v: *mut vnode, sizeofbuf:usize, offset: usize, buf: *mut u64, rw: i32);
    fn v_lookup(&mut self, v: *mut vnode, part: &str, l: *mut *mut vnode);
}
trait vfsops {

}
