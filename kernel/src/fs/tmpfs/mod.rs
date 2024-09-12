use super::vfs::vnode;
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
struct tmpfsnode {
    data: *mut [u8],
    size: usize
}
struct tmpfsdirentry {
    name: String,
    vnode: vnode,
    next: Box<tmpfsdirentry>
}