use core::cell::RefCell;
extern crate alloc;
use crate::utils::UNIXERROR;
use alloc::rc::Rc;
use vfs::{resolve_path_absolute, vnode};

pub mod USTAR;
pub mod tmpfs;
pub mod vfs;
trait Stream {
    fn read(&self, buf: &mut [u8], sizeofbut: usize) -> Result<usize, UNIXERROR>;
    fn write(&self, buf: &[u8], sizeofbut: usize) -> Result<usize, UNIXERROR>;
    fn seek(&mut self, offset: isize, wh: WHENCE) -> Result<usize, UNIXERROR>;
}
struct VNodeStream {
    offset: usize,
    vnode: Rc<RefCell<dyn vnode>>,
}
#[derive(Clone)]
pub enum VNODEFLAGS {
    None,
    DIR,
    SYMLINK,
    FILE,
}
pub enum WHENCE {
    CURRENT,
    SET,
    END,
}
impl Stream for VNodeStream {
    fn read(&self, buf: &mut [u8], sizeofbut: usize) -> Result<usize, UNIXERROR> {
        let res = self.vnode.borrow_mut().read(buf, self.offset, sizeofbut);
        return res;
    }
    fn seek(&mut self, offset: isize, wh: WHENCE) -> Result<usize, UNIXERROR> {
        match wh {
            WHENCE::CURRENT => {
                self.offset = (self.offset as isize + offset) as usize;
            }
            WHENCE::SET => self.offset = (self.offset as usize),
            WHENCE::END => {
                self.offset =
                    (self.vnode.borrow_mut().get_attrib().unwrap().size as isize + offset) as usize
            }
        }
        Ok(self.offset)
    }
    fn write(&self, buf: &[u8], sizeofbut: usize) -> Result<usize, UNIXERROR> {
        let res = self.vnode.borrow_mut().write(buf, self.offset, sizeofbut);
        return res;
    }
}
pub struct PosixFile {
    vnode: Rc<RefCell<dyn vnode>>,
    flags: VNODEFLAGS,
    stream: VNodeStream,
}
impl PosixFile {
    pub fn open(path: &str) -> Result<Self, UNIXERROR> {
        let f: Result<Rc<RefCell<dyn vnode>>, UNIXERROR> = resolve_path_absolute(path, false);

        match f {
            Ok(h) => {
                let fla = h.borrow_mut().get_attrib().unwrap().TYPE.clone();
                return Ok(PosixFile {
                    vnode: h.clone(),
                    flags: fla,
                    stream: VNodeStream {
                        offset: 0,
                        vnode: h.clone(),
                    },
                });
            }
            Err(e) => return Err(e),
        }
    }
    pub fn openat(fd: PosixFile, path: &str) {
        todo!()
    }
    pub fn seek(&mut self, offset: isize, when: WHENCE) -> Result<usize, UNIXERROR> {
        let res = self.stream.seek(offset, when);
        if res.is_err() {
            return Err(res.unwrap_err());
        }
        return Ok(res.unwrap());
    }
    pub fn write(&mut self, buf: &[u8], count: usize) -> Result<usize, UNIXERROR> {
        let res = self.stream.write(buf, count);
        if res.is_err() {
            return Err(res.unwrap_err());
        }
        return Ok(res.unwrap());
    }
}
