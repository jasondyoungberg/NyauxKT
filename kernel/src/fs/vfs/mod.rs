extern crate alloc;
use core::ops::Index;

use alloc::boxed::Box;
use alloc::string::ToString;
use crate::println;
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
pub fn get_rest_of_path(path: alloc::string::String) -> alloc::string::String {
    // Remove any leading '/'
    let trimmed_path = if path.char_indices().next().unwrap().1 == '/'
    {
        Some(path.trim_start_matches('/'))
        
    }
    else {
        Some(path.as_str())
    }.unwrap();
    alloc::string::String::new()
    
   
    
}

pub fn get_word(path: alloc::string::String) -> alloc::string::String {
    // Convert the input String to a slice for easier manipulation
    let trimmed_path = if path.char_indices().next().unwrap().1 == '/'
    {
        Some(path.trim_start_matches('/'))
        
    }
    else {
        Some(path.as_str())
    }.unwrap();
    // Find the position of the first '/'
    match trimmed_path.find('/') {
        Some(pos) => {
            // If a '/' is found, return the part before it
            trimmed_path[..pos].to_string()
        }
        None => {
            // If no '/' is found, return the whole path
            trimmed_path.to_string()
        }
    }
}
