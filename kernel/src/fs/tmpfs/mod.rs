use core::alloc::Layout;
use core::iter::Map;
use core::ptr::addr_of_mut;



use crate::utils::UNIXERROR;


extern crate alloc;
use alloc::rc::Rc;
use hashbrown::HashMap;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::string::String as privatetype;

use super::vfs;
// struct tmpfsfile {
//     data: Option<*mut u8>,
//     size: usize
// }
// pub struct tmpfsdirentry {
//     name: String,
//     vnode: vnode,
//     next: Option<Box<tmpfsdirentry>>
// }
// pub struct tmpfsdir {
//     pub head: Option<Box<tmpfsdirentry>>
// }
pub struct tmpfsdir {
    files: HashMap<String, Rc<RefCell<dyn vfs::vnode>>>
}
pub struct tmpfsfile {
    data: alloc::vec::Vec<u8>,
    symlink: bool
}
impl Default for tmpfsdir {
    fn default() -> Self {
        Self {
            files: HashMap::new(),
        }
    }
}
impl<'a> Default for tmpfsfile {
    fn default() -> Self {
        Self {
            data: alloc::vec::Vec::new(),
            symlink: false
        }
    }
}
use core::cell::RefCell;
impl vfs::vnode for tmpfsfile {
    fn create(&mut self, name: &str) -> Result<Rc<RefCell<dyn vfs::vnode>>, UNIXERROR> {
        return Err(UNIXERROR::EISFILE);
    }
    fn lookup(&self, child: &str) -> Result<Rc<RefCell<dyn vfs::vnode>>, UNIXERROR> {
        return Err(UNIXERROR::EISFILE);
    }
    fn mkdir(&mut self, name: &str) -> Result<Rc<RefCell<dyn vfs::vnode>>, UNIXERROR> {
        return Err(UNIXERROR::EISFILE);
    }
    fn read(&self, buf: &mut [u8], offset: usize) -> Result<usize, UNIXERROR> {
        let mut size = offset + buf.len();
        if buf.len() == 1 {
            return Err(UNIXERROR::EINVAL);
        }
        if offset + buf.len() >= self.data.len() {
            size = self.data.len();
        }
        if self.data.len() == 0 {
            return Err(UNIXERROR::EPERM);
        }
        let pro = &self.data[offset..size];
        buf[..pro.len()].copy_from_slice(pro);
        return Ok(size);
    }
    fn write(&mut self, buf: &[u8], offset: usize) -> Result<usize, UNIXERROR> {
        let size = offset + buf.len();
        
        if offset + buf.len() > self.data.len() {
            
            self.data.resize(offset + buf.len(), 0);
        }
        if self.data.len() == 0 {
            return Err(UNIXERROR::EPERM);
        }
        self.data.resize(size, 0);
        self.data.copy_from_slice(&buf[offset..size]);
        // SO EASY
        return Ok(size);
    }
    fn is_symlink(&self) -> Result<bool, UNIXERROR> {
        Ok(self.symlink)
    }
    fn set_symlink(&mut self, bo: bool) -> Result<(), UNIXERROR> {
        self.symlink = bo;
        Ok(())
    }
}
use alloc::string::ToString;
impl vfs::vnode for tmpfsdir {
    fn create(&mut self, name: &str) -> Result<Rc<RefCell<dyn vfs::vnode>>, UNIXERROR> {
        let dir = &mut self.files;
        let ifeelsick = Rc::new(RefCell::new(tmpfsfile::default()));
        dir.insert(name.to_string(), ifeelsick.clone());
        return Ok(ifeelsick);

    }
    fn lookup(&self, child: &str) -> Result<Rc<RefCell<dyn vfs::vnode>>, UNIXERROR> {
        if self.files.contains_key(child) {
            if let Some(qt) = self.files.get(child) {
                return Ok(qt.clone());
            }
        }
        return Err(UNIXERROR::EISDIR);
    }
    fn mkdir(&mut self, name: &str) -> Result<Rc<RefCell<dyn vfs::vnode>>, UNIXERROR> {
        let dir = &mut self.files;
        let ifeelsick = Rc::new(RefCell::new(tmpfsdir::default()));
        dir.insert(String::from(name), ifeelsick.clone());
        return Ok(ifeelsick);
        
    }
    fn read(&self, buf: &mut [u8], offset: usize) -> Result<usize, UNIXERROR> {
        return Err(UNIXERROR::EISDIR);
    }
    fn write(&mut self, buf: &[u8], offset: usize) -> Result<usize, UNIXERROR> {
        return Err(UNIXERROR::EISDIR);
    }
    fn is_symlink(&self) -> Result<bool, UNIXERROR> {
        return Err(UNIXERROR::EISDIR);
    }
    fn set_symlink(&mut self, bo: bool) -> Result<(), UNIXERROR> {
        return Err(UNIXERROR::EISDIR);
    }
}
// pub struct tmpfsreal;

// impl Vnodeops for tmpfsreal
// {
//     fn v_filesz(&mut self, v: *mut vnode) -> Result<usize, UNIXERROR> {
//         unsafe {
//             match (*v).flags
//         {
//             super::vfs::vnodetype::DIRECTORY => {
//                 return Err(UNIXERROR::EISDIR); // EISDIR
//             },
//             super::vfs::vnodetype::FILE => {
//                 let file = (*v).data as *mut tmpfsfile;
//                 unsafe {
//                     return Ok((*file).size);
//                 }

//             },
//             super::vfs::vnodetype::SYMLINK => {
//                 return Err(UNIXERROR::ENOENT)
//             }
//         }
//         }
        
//     }
//     fn v_lookup(&mut self, v: *mut vnode, part: &str, l: &mut Option<&mut vnode>) -> UNIXERROR {
//         unsafe {
//             match (*v).flags {
//                 super::vfs::vnodetype::DIRECTORY => {
//                     return UNIXERROR::EISDIR; // EISDIR
//                 }
//                 super::vfs::vnodetype::FILE => {
//                     let dir = (*v).data as *mut tmpfsdir;
                    
//                         let mut current = (*dir).head.as_mut(); // Start with the head of the list
//                         while let Some(t) = current {
//                             if t.name == part {
//                                 *l = Some(&mut t.vnode);
//                                 return UNIXERROR::ESUCCESS;
//                             }
//                             current = t.next.as_mut(); // Move to the next node in the list
//                         }
//                         return UNIXERROR::ENOENT;
                    
//                 }
//                 super::vfs::vnodetype::SYMLINK => {
//                     return UNIXERROR::ENOENT;
//                 }
//             }
//         }
//         }
            
//     fn v_rdwr(&mut self, v: *mut vnode, sizeofbuf: usize, offset: usize, buf: &mut u8, rw: i32) -> Result<usize, UNIXERROR> {
//         match rw {
//             0 => {
//                 // read
//                 let file = unsafe {(*v).data as *mut tmpfsfile};
//                 unsafe {
//                     if (*file).size == 0 {
//                         return Err(UNIXERROR::ENOENT);
//                     }
//                     let mut start = (*file).data.unwrap() as u64;
//                     let mut end = start + sizeofbuf as u64;
//                     if end > (*file).data.unwrap() as u64 + (*file).size as u64 {
//                         end = (*file).data.unwrap() as u64 + (*file).size as u64;
//                     }
//                     if start > end {
//                         // how is this possible lol
//                         start = end;

//                     }
//                     (*file).data.unwrap().copy_to(*buf as *mut u8, ((end - start) as usize) / 8);
//                     return Ok((end - start) as usize);
//                 }
//             },
//             1 => {
//                 // write
//                 let file = unsafe {(*v).data as *mut tmpfsfile};
//                 unsafe {
//                     if (*file).size == 0 {
//                         // allocate
//                         // align doesnt matter anyway
//                         (*file).data = Some(alloc::alloc::alloc_zeroed(Layout::from_size_align(sizeofbuf + offset, 4096).unwrap()));
//                         (*file).size = sizeofbuf + offset;
    
//                     }
//                     else if sizeofbuf + offset > (*file).size {
//                         (*file).data = Some(alloc::alloc::realloc(
//                             (*file).data.unwrap()
//                             , Layout::from_size_align((*file).size, 4096).unwrap(), sizeofbuf + offset));
//                     }
//                     (*file).data.unwrap().copy_from(*buf as *mut u8, sizeofbuf);
//                     return Ok(sizeofbuf);
//                 }
                
//             },
//             _ => {
//                 return Err(UNIXERROR::EINVAL); // Invalid Arugment
//             }
//         }
        
//     }
//     fn v_create(&mut self, v: *mut vnode, name: &str, result: &mut Option<*mut vnode>) -> UNIXERROR {
//         use alloc::string::ToString;
//         unsafe {
//             if (*v).flags == vnodetype::DIRECTORY {
//                 let dir = (*v).data as *mut tmpfsdir;
                
//                     match &(*dir).head {
//                         Some(_) => {
//                             let mut o = Box::new(tmpfsdirentry {
//                                 name: name.to_string(),
//                                 next: None,
//                                 vnode: 
//                                     vnode {
//                                         ops: Box::new(tmpfsreal),
//                                         flags: vnodetype::FILE,
//                                         data: Box::into_raw(Box::new(tmpfsfile 
//                                         {
//                                             size: 0,
//                                             data: None
//                                         })) as *mut u8
//                                     }
//                                 }
//                                 );
//                                 o.next = Some((*dir).head.take().unwrap());
//                                 *result = Some(&mut o.vnode as *mut vnode);
//                                 (*dir).head = Some(o);
                                
//                                 return UNIXERROR::ESUCCESS;
                            
//                             },
                        
//                         None => {
//                             let mut o = Box::new(tmpfsdirentry {
//                                 name: name.to_string(),
//                                 next: None,
//                                 vnode: 
//                                     vnode {
//                                         ops: Box::new(tmpfsreal),
//                                         flags: vnodetype::FILE,
//                                         data: Box::into_raw(Box::new(tmpfsfile 
//                                         {
//                                             size: 0,
//                                             data: None
//                                         })) as *mut u8
//                                     }
//                                 }
//                                 );
                                
                                
//                                 *result = Some(addr_of_mut!(o.vnode) as *mut vnode);
//                                 (*dir).head = Some(o);
//                                 return UNIXERROR::ESUCCESS;
//                         }
//                     }
                
//             }
//             else {
//                 return UNIXERROR::EINVAL;
//             }
//         }
        
//     }
//     fn v_mkdir(&mut self, v: *mut vnode, name: &str, resilt: &mut Option<*mut vnode>) -> UNIXERROR {
//         use alloc::string::ToString;
//         unsafe {
//             if (*v).flags == vnodetype::DIRECTORY {
//                 let dir = (*v).data as *mut tmpfsdir;
               
//                     match &(*dir).head {
//                         Some(_) => {
//                             let mut o = Box::new(tmpfsdirentry {
//                                 name: name.to_string(),
//                                 next: None,
//                                 vnode: 
//                                     vnode {
//                                         ops: Box::new(tmpfsreal),
//                                         flags: vnodetype::DIRECTORY,
//                                         data: Box::into_raw(Box::new(tmpfsdir 
//                                         {
//                                             head: None
//                                         })) as *mut u8
//                                     }
//                                 }
//                                 );
//                                 o.next = Some((*dir).head.take().unwrap());
//                                 *resilt = Some(&mut o.vnode as *mut vnode);
//                                 (*dir).head = Some(o);
                                
//                                 return UNIXERROR::ESUCCESS;
                            
//                             },
                        
//                         None => {
//                             let mut o = Box::new(tmpfsdirentry {
//                                 name: name.to_string(),
//                                 next: None,
//                                 vnode: 
//                                     vnode {
//                                         ops: Box::new(tmpfsreal),
//                                         flags: vnodetype::DIRECTORY,
//                                         data: Box::into_raw(Box::new(tmpfsdir 
//                                         {
//                                             head: None
//                                         })) as *mut u8
//                                     }
//                                 }
//                                 );
                                
                                
//                                 *resilt = Some(&mut o.vnode as *mut vnode);
//                                 (*dir).head = Some(o);
//                                 return UNIXERROR::ESUCCESS;
//                         }
//                     }
//                 }
//             else {
//                 return UNIXERROR::EINVAL;
//             }
//         }
        
//     }
//     fn v_getdataraw(&mut self, v: *mut vnode) -> Option<*mut u8> {
//         unsafe {
//             match (*v).flags {
//                 vnodetype::DIRECTORY => {
//                     return None;
//                 },
//                 vnodetype::FILE => {
//                     let oo = (*v).data as *mut tmpfsfile;
//                     return (*oo).data;
//                 },
//                 vnodetype::SYMLINK => {
//                     return None;
//                 }
//             }
//         }
//     }
// }