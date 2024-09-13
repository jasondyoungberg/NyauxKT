use core::alloc::Layout;

use crate::utils::UNIXERROR;

use super::vfs::{vnode, Vnodeops};
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
struct tmpfsfile {
    data: *mut u8,
    size: usize
}
struct tmpfsdirentry {
    name: String,
    vnode: vnode,
    next: Option<Box<tmpfsdirentry>>
}
struct tmpfsdir {
    head: Option<*mut tmpfsdirentry>
}
trait tmpfs
{

}
impl Vnodeops for dyn tmpfs
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
    fn v_lookup(&mut self, v: &mut vnode, part: &str, mut l: &mut Option<&mut vnode>) -> UNIXERROR {
        match v.flags
        {
            super::vfs::vnodetype::DIRECTORY => {
                return UNIXERROR::EISDIR; // EISDIR
            },
            super::vfs::vnodetype::FILE => {
                let dir = v.data as *mut tmpfsdir;
                unsafe {
                    match (*dir).head
                    {
                        Some(t) => {
                            if (*t).name == part {
                                *l = Some(&mut (*t).vnode);
                                return UNIXERROR::ESUCCESS;
                                
                            }
                            while let Some(check) = &mut (*t).next
                            {
                                if check.name == part {
                                    *l = Some(&mut (check).vnode);
                                    return UNIXERROR::ESUCCESS;
                                }
                            }
                            return UNIXERROR::ENOENT;
                        },
                        None => {
                            return UNIXERROR::ENOENT;
                        }
                    }
                }

            },
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
                    let mut start = (*file).data as u64;
                    let mut end = start + sizeofbuf as u64;
                    if end > (*file).data as u64 + (*file).size as u64 {
                        end = (*file).data as u64 + (*file).size as u64;
                    }
                    if start > end {
                        // how is this possible lol
                        start = end;

                    }
                    (*file).data.copy_to(*buf as *mut u8, ((end - start) as usize) / 8);
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
                        (*file).data = alloc::alloc::alloc_zeroed(Layout::from_size_align(sizeofbuf + offset, 4096).unwrap());
                        (*file).size = sizeofbuf + offset;
    
                    }
                    else if sizeofbuf + offset > (*file).size {
                        (*file).data = alloc::alloc::realloc(
                            (*file).data
                            , Layout::from_size_align((*file).size, 4096).unwrap(), sizeofbuf + offset);
                    }
                    (*file).data.copy_from(*buf as *mut u8, sizeofbuf);
                    return Ok(sizeofbuf);
                }
                
            },
            _ => {
                return Err(UNIXERROR::EINVAL); // Invalid Arugment
            }
        }
    }
}