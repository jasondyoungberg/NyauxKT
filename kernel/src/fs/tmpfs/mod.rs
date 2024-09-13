use core::alloc::Layout;

use crate::utils::UNIXERROR;

use super::vfs::{vnode, vnodetype, Vnodeops};
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
struct tmpfsfile {
    data: Option<*mut u8>,
    size: usize
}
struct tmpfsdirentry {
    name: String,
    vnode: vnode,
    next: Option<Box<tmpfsdirentry>>
}
struct tmpfsdir {
    head: Option<Box<tmpfsdirentry>>
}

struct tmpfsreal;

impl Vnodeops for tmpfsreal
{
    fn v_filesz(&mut self, v: &mut vnode) -> Result<usize, UNIXERROR> {
        match v.flags
        {
            super::vfs::vnodetype::DIRECTORY => {
                return Err(UNIXERROR::EISDIR); // EISDIR
            },
            super::vfs::vnodetype::FILE => {
                let file = v.data as *mut tmpfsfile;
                unsafe {
                    return Ok((*file).size);
                }

            },
            super::vfs::vnodetype::SYMLINK => {
                return Err(UNIXERROR::ENOENT)
            }
        }
    }
    fn v_lookup(&mut self, v: &mut vnode, part: &str, l: &mut Option<&mut vnode>) -> UNIXERROR {
        match v.flags {
            super::vfs::vnodetype::DIRECTORY => {
                return UNIXERROR::EISDIR; // EISDIR
            }
            super::vfs::vnodetype::FILE => {
                let dir = v.data as *mut tmpfsdir;
                unsafe {
                    let mut current = (*dir).head.as_mut(); // Start with the head of the list
                    while let Some(t) = current {
                        if t.name == part {
                            *l = Some(&mut t.vnode);
                            return UNIXERROR::ESUCCESS;
                        }
                        current = t.next.as_mut(); // Move to the next node in the list
                    }
                    return UNIXERROR::ENOENT;
                }
            }
            super::vfs::vnodetype::SYMLINK => {
                return UNIXERROR::ENOENT;
            }
        }
    }    
    fn v_rdwr(&mut self, v: &mut vnode, sizeofbuf: usize, offset: usize, buf: &mut u8, rw: i32) -> Result<usize, UNIXERROR> {
        match rw {
            0 => {
                // read
                let file = v.data as *mut tmpfsfile;
                unsafe {
                    if (*file).size == 0 {
                        return Err(UNIXERROR::ENOENT);
                    }
                    let mut start = (*file).data.unwrap() as u64;
                    let mut end = start + sizeofbuf as u64;
                    if end > (*file).data.unwrap() as u64 + (*file).size as u64 {
                        end = (*file).data.unwrap() as u64 + (*file).size as u64;
                    }
                    if start > end {
                        // how is this possible lol
                        start = end;

                    }
                    (*file).data.unwrap().copy_to(*buf as *mut u8, ((end - start) as usize) / 8);
                    return Ok((end - start) as usize);
                }
            },
            1 => {
                // write
                let file = v.data as *mut tmpfsfile;
                unsafe {
                    if (*file).size == 0 {
                        // allocate
                        // align doesnt matter anyway
                        (*file).data = Some(alloc::alloc::alloc_zeroed(Layout::from_size_align(sizeofbuf + offset, 4096).unwrap()));
                        (*file).size = sizeofbuf + offset;
    
                    }
                    else if sizeofbuf + offset > (*file).size {
                        (*file).data = Some(alloc::alloc::realloc(
                            (*file).data.unwrap()
                            , Layout::from_size_align((*file).size, 4096).unwrap(), sizeofbuf + offset));
                    }
                    (*file).data.unwrap().copy_from(*buf as *mut u8, sizeofbuf);
                    return Ok(sizeofbuf);
                }
                
            },
            _ => {
                return Err(UNIXERROR::EINVAL); // Invalid Arugment
            }
        }
        
    }
    fn v_create(&mut self, v: &mut vnode, name: &str, result: &mut Option<*mut vnode>) -> UNIXERROR {
        use alloc::string::ToString;
        if v.flags == vnodetype::DIRECTORY {
            let dir = v.data as *mut tmpfsdir;
            unsafe {
                match &(*dir).head {
                    Some(_) => {
                        let mut o = Box::new(tmpfsdirentry {
                            name: name.to_string(),
                            next: None,
                            vnode: 
                                vnode {
                                    ops: Box::new(tmpfsreal),
                                    flags: vnodetype::FILE,
                                    data: Box::into_raw(Box::new(tmpfsfile 
                                    {
                                        size: 0,
                                        data: None
                                    })) as *mut u8
                                }
                            }
                            );
                            o.next = Some((*dir).head.take().unwrap());
                            *result = Some(&mut o.vnode as *mut vnode);
                            (*dir).head = Some(o);
                            
                            return UNIXERROR::ESUCCESS;
                        
                        },
                    
                    None => {
                        let mut o = Box::new(tmpfsdirentry {
                            name: name.to_string(),
                            next: None,
                            vnode: 
                                vnode {
                                    ops: Box::new(tmpfsreal),
                                    flags: vnodetype::FILE,
                                    data: Box::into_raw(Box::new(tmpfsfile 
                                    {
                                        size: 0,
                                        data: None
                                    })) as *mut u8
                                }
                            }
                            );
                            
                            
                            *result = Some(&mut o.vnode as *mut vnode);
                            (*dir).head = Some(o);
                            return UNIXERROR::ESUCCESS;
                    }
                }
            }
        }
        else {
            return UNIXERROR::EINVAL;
        }
    }
    fn v_mkdir(&mut self, v: &mut vnode, name: &str, resilt: &mut Option<*mut vnode>) -> UNIXERROR {
        use alloc::string::ToString;
        if v.flags == vnodetype::DIRECTORY {
            let dir = v.data as *mut tmpfsdir;
            unsafe {
                match &(*dir).head {
                    Some(_) => {
                        let mut o = Box::new(tmpfsdirentry {
                            name: name.to_string(),
                            next: None,
                            vnode: 
                                vnode {
                                    ops: Box::new(tmpfsreal),
                                    flags: vnodetype::DIRECTORY,
                                    data: Box::into_raw(Box::new(tmpfsdir 
                                    {
                                        head: None
                                    })) as *mut u8
                                }
                            }
                            );
                            o.next = Some((*dir).head.take().unwrap());
                            *resilt = Some(&mut o.vnode as *mut vnode);
                            (*dir).head = Some(o);
                            
                            return UNIXERROR::ESUCCESS;
                        
                        },
                    
                    None => {
                        let mut o = Box::new(tmpfsdirentry {
                            name: name.to_string(),
                            next: None,
                            vnode: 
                                vnode {
                                    ops: Box::new(tmpfsreal),
                                    flags: vnodetype::DIRECTORY,
                                    data: Box::into_raw(Box::new(tmpfsdir 
                                    {
                                        head: None
                                    })) as *mut u8
                                }
                            }
                            );
                            
                            
                            *resilt = Some(&mut o.vnode as *mut vnode);
                            (*dir).head = Some(o);
                            return UNIXERROR::ESUCCESS;
                    }
                }
            }
        }
        else {
            return UNIXERROR::EINVAL;
        }
    }
}